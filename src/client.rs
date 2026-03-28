use crate::error::{Error, Result};
use reqwest::{Client, Proxy, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use tracing::{debug, warn};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MultipartFile {
    pub bytes: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

impl MultipartFile {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
            filename: None,
            mime_type: None,
        }
    }

    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    pub fn with_mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }

    fn into_part(self) -> reqwest::multipart::Part {
        let mut part = reqwest::multipart::Part::bytes(self.bytes);
        if let Some(filename) = self.filename {
            part = part.file_name(filename);
        }
        part
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultipartField {
    Text(String),
    File(MultipartFile),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MultipartForm {
    fields: Vec<(String, MultipartField)>,
}

impl MultipartForm {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn text(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.fields
            .push((name.into(), MultipartField::Text(value.into())));
    }

    pub fn file(&mut self, name: impl Into<String>, file: MultipartFile) {
        self.fields.push((name.into(), MultipartField::File(file)));
    }

    fn into_reqwest(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        for (name, field) in self.fields {
            form = match field {
                MultipartField::Text(value) => form.text(name, value),
                MultipartField::File(file) => form.part(name, file.into_part()),
            };
        }
        form
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestBody {
    Json(serde_json::Value),
    Form(Vec<(String, String)>),
    Multipart(MultipartForm),
}

impl RequestBody {
    fn into_builder(self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match self {
            RequestBody::Json(value) => req.json(&value),
            RequestBody::Form(fields) => req.form(&fields),
            RequestBody::Multipart(form) => req.multipart(form.into_reqwest()),
        }
    }
}

fn parse_retry_after(value: &str) -> Option<Duration> {
    if let Ok(secs) = value.parse::<u64>() {
        return Some(Duration::from_secs(secs));
    }

    let at = httpdate::parse_http_date(value).ok()?;
    Some(
        at.duration_since(SystemTime::now())
            .unwrap_or(Duration::ZERO),
    )
}

fn apply_body(req: reqwest::RequestBuilder, body: RequestBody) -> reqwest::RequestBuilder {
    body.into_builder(req)
}

const DEFAULT_MAX_RETRIES: u32 = 5;
const INITIAL_BACKOFF: Duration = Duration::from_secs(2);
const MAX_BACKOFF: Duration = Duration::from_secs(60);

fn extract_error_message(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| {
            v.get("error")
                .and_then(|e| e.get("message"))
                .or_else(|| v.get("error"))
                .or_else(|| v.get("message"))
                .and_then(|m| m.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| body.to_string())
}

fn is_retryable_transport_error(error: &reqwest::Error) -> bool {
    error.is_timeout() || error.is_connect()
}

fn is_retryable_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}

#[derive(Debug, Clone)]
pub struct ApiClientBuilder {
    base_url: String,
    token: String,
    proxy: Option<String>,
    max_retries: u32,
    timeout: Duration,
}

impl ApiClientBuilder {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            token: token.into(),
            proxy: None,
            max_retries: DEFAULT_MAX_RETRIES,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.proxy = Some(proxy_url.into());
        self
    }

    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Result<ApiClient> {
        let mut headers = reqwest::header::HeaderMap::new();
        if !self.token.is_empty() {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.token)
                    .parse()
                    .map_err(|_| Error::Api {
                        status: 0,
                        body: "invalid token: contains characters not allowed in HTTP headers"
                            .into(),
                    })?,
            );
        }
        headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());

        let mut builder = Client::builder()
            .timeout(self.timeout)
            .default_headers(headers);

        if let Some(proxy_url) = &self.proxy {
            builder = builder.proxy(Proxy::all(proxy_url)?);
        }

        Ok(ApiClient {
            http: builder.build()?,
            base_url: self.base_url,
            max_retries: self.max_retries,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub(crate) http: Client,
    pub(crate) base_url: String,
    pub(crate) max_retries: u32,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Result<Self> {
        ApiClientBuilder::new(base_url, token).build()
    }

    pub fn builder(base_url: impl Into<String>, token: impl Into<String>) -> ApiClientBuilder {
        ApiClientBuilder::new(base_url, token)
    }

    /// Выполняет запрос с авто-ретраем на 429/502/503/504 и транзиентные ошибки сети.
    pub async fn request<Q, R>(
        &self,
        method: &str,
        path: &str,
        query: Option<&Q>,
        body: Option<RequestBody>,
    ) -> Result<R>
    where
        Q: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!(
                "{}/{}",
                self.base_url.trim_end_matches('/'),
                path.trim_start_matches('/')
            )
        };

        let mut backoff = INITIAL_BACKOFF;

        for attempt in 0..=self.max_retries {
            let mut req = match method {
                "get" => self.http.get(&url),
                "post" => self.http.post(&url),
                "put" => self.http.put(&url),
                "delete" => self.http.delete(&url),
                "patch" => self.http.patch(&url),
                other => {
                    return Err(Error::Api {
                        status: 0,
                        body: format!("unsupported HTTP method: {}", other),
                    })
                }
            };

            if let Some(q) = query {
                req = req.query(q);
            }
            if let Some(b) = body.clone() {
                req = apply_body(req, b);
            }

            debug!(attempt, method, url = %url, "sending request");

            let resp = match req.send().await {
                Ok(r) => r,
                Err(e) if attempt < self.max_retries && is_retryable_transport_error(&e) => {
                    warn!(attempt, error = %e, "transport error, retrying in {:?}", backoff);
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                    continue;
                }
                Err(e) => return Err(Error::Http(e)),
            };

            let status = resp.status();
            if is_retryable_status(status) && attempt < self.max_retries {
                let retry_after = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(parse_retry_after);

                let wait = retry_after.unwrap_or(backoff);
                warn!(
                    attempt,
                    status = status.as_u16(),
                    "retryable status, waiting {:?}",
                    wait
                );
                tokio::time::sleep(wait).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
                continue;
            }

            let status_code = status.as_u16();
            let response_text = resp.text().await.map_err(Error::Http)?;

            if status == StatusCode::TOO_MANY_REQUESTS {
                return Err(Error::RateLimited {
                    attempts: self.max_retries + 1,
                });
            }

            if !status.is_success() {
                let message = extract_error_message(&response_text);
                return Err(match status_code {
                    401 => Error::Auth { message },
                    403 => Error::Forbidden { message },
                    404 => Error::NotFound { message },
                    _ => Error::Api {
                        status: status_code,
                        body: response_text,
                    },
                });
            }

            let parsed: R = serde_json::from_str(&response_text).map_err(Error::Json)?;
            return Ok(parsed);
        }

        Err(Error::RateLimited {
            attempts: self.max_retries + 1,
        })
    }
}
