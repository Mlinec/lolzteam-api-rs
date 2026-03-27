use crate::error::{Error, Result};
use reqwest::{Client, Proxy, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::{Duration, SystemTime};
use tracing::{debug, warn};

#[derive(Debug)]
pub enum RequestBody {
    Json(serde_json::Value),
    Form(Vec<(String, String)>),
    Multipart(reqwest::multipart::Form),
}

impl RequestBody {
    fn into_builder(self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match self {
            RequestBody::Json(value) => req.json(&value),
            RequestBody::Form(fields) => req.form(&fields),
            RequestBody::Multipart(form) => req.multipart(form),
        }
    }
}

impl Clone for RequestBody {
    fn clone(&self) -> Self {
        match self {
            RequestBody::Json(value) => RequestBody::Json(value.clone()),
            RequestBody::Form(fields) => RequestBody::Form(fields.clone()),
            RequestBody::Multipart(_) => panic!("multipart request bodies cannot be cloned"),
        }
    }
}

impl PartialEq for RequestBody {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RequestBody::Json(a), RequestBody::Json(b)) => a == b,
            (RequestBody::Form(a), RequestBody::Form(b)) => a == b,
            (RequestBody::Multipart(_), RequestBody::Multipart(_)) => true,
            _ => false,
        }
    }
}

impl Eq for RequestBody {}

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
        let mut builder = Client::builder().timeout(self.timeout).default_headers({
            let mut h = reqwest::header::HeaderMap::new();
            h.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.token)
                    .parse()
                    .map_err(|_| Error::Api {
                        status: 0,
                        body: "invalid token: contains characters not allowed in HTTP headers"
                            .into(),
                    })?,
            );
            h.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
            h
        });

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

    /// Выполняет запрос с авто-ретраем на 429/502/503.
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
                Err(e) if attempt < self.max_retries && e.is_timeout() => {
                    warn!(attempt, "timeout, retrying in {:?}", backoff);
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                    continue;
                }
                Err(e) => return Err(Error::Http(e)),
            };

            let status = resp.status();

            if matches!(
                status,
                StatusCode::TOO_MANY_REQUESTS
                    | StatusCode::BAD_GATEWAY
                    | StatusCode::SERVICE_UNAVAILABLE
            ) {
                if attempt < self.max_retries {
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

                return Err(Error::RateLimited {
                    attempts: self.max_retries + 1,
                });
            }

            let status_code = status.as_u16();
            let response_text = resp.text().await.map_err(Error::Http)?;

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
