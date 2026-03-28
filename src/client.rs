use crate::error::{Error, Result};
use reqwest::{Client, Proxy, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::Mutex;
use tracing::{debug, warn};

// ── Multipart helpers ──

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

// ── Request body ──

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

// ── Retry info (passed to on_retry callback) ──

/// Information passed to the `on_retry` callback on each retry attempt.
#[derive(Debug, Clone)]
pub struct RetryInfo {
    /// The retry attempt number (1-based).
    pub attempt: u32,
    /// The delay in milliseconds before this retry.
    pub delay_ms: u64,
    /// The HTTP method of the request.
    pub method: String,
    /// The URL path of the request.
    pub path: String,
    /// The status code that caused the retry, if available.
    pub status: Option<u16>,
}

// ── Token bucket rate limiter ──

#[derive(Debug)]
struct BucketState {
    tokens: f64,
    last_refill: Instant,
}

/// A token-bucket rate limiter.
///
/// Allows up to `requests_per_minute` requests per minute, with burst capacity
/// equal to the per-minute limit. Uses continuous refill based on elapsed time.
#[derive(Debug)]
pub struct RateLimiter {
    state: Mutex<BucketState>,
    tokens_per_sec: f64,
    max_tokens: f64,
}

impl RateLimiter {
    /// Create a new rate limiter with the given requests-per-minute limit.
    pub fn new(requests_per_minute: u32) -> Self {
        let max_tokens = requests_per_minute as f64;
        Self {
            state: Mutex::new(BucketState {
                tokens: max_tokens,
                last_refill: Instant::now(),
            }),
            tokens_per_sec: max_tokens / 60.0,
            max_tokens,
        }
    }

    /// Wait until a token is available, then consume it.
    pub async fn acquire(&self) {
        loop {
            let sleep_dur = {
                let mut state = self.state.lock().await;
                let now = Instant::now();
                let elapsed = now.duration_since(state.last_refill).as_secs_f64();
                state.tokens = (state.tokens + elapsed * self.tokens_per_sec).min(self.max_tokens);
                state.last_refill = now;

                if state.tokens >= 1.0 {
                    state.tokens -= 1.0;
                    return;
                }
                // Calculate time to wait for 1 token
                let deficit = 1.0 - state.tokens;
                Duration::from_secs_f64(deficit / self.tokens_per_sec)
            };
            tokio::time::sleep(sleep_dur).await;
        }
    }
}

// ── Retry / backoff helpers ──

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
const DEFAULT_BASE_DELAY_MS: u64 = 1000;
const DEFAULT_MAX_DELAY_MS: u64 = 60_000;

/// Compute backoff delay: `min(base_delay * 2^attempt + jitter, max_delay)`.
fn compute_backoff(base_delay_ms: u64, max_delay_ms: u64, attempt: u32) -> Duration {
    let base = base_delay_ms.saturating_mul(1u64.wrapping_shl(attempt));
    // Add jitter: random(0, base_delay_ms)
    let jitter = fastrand_u64() % base_delay_ms.max(1);
    let total = base.saturating_add(jitter).min(max_delay_ms);
    Duration::from_millis(total)
}

/// Simple pseudo-random u64 using thread-local state (no external crate needed).
fn fastrand_u64() -> u64 {
    use std::cell::Cell;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    thread_local! {
        static STATE: Cell<u64> = Cell::new({
            let mut h = DefaultHasher::new();
            std::thread::current().id().hash(&mut h);
            // Mix in current time for more entropy
            if let Ok(d) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                d.as_nanos().hash(&mut h);
            }
            h.finish()
        });
    }

    STATE.with(|s| {
        // xorshift64
        let mut x = s.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        s.set(x);
        x
    })
}

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

// ── Retry configuration ──

/// Retry policy for transient errors (429, 502, 503, 504).
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retries (0 = no retries).
    pub max_retries: u32,
    /// Initial backoff delay in milliseconds.
    pub base_delay_ms: u64,
    /// Maximum backoff delay in milliseconds.
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
            base_delay_ms: DEFAULT_BASE_DELAY_MS,
            max_delay_ms: DEFAULT_MAX_DELAY_MS,
        }
    }
}

/// Rate limiting configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per minute.
    pub requests_per_minute: u32,
}

// ── ApiClient builder ──

/// Builder for constructing an [`ApiClient`] with custom configuration.
#[derive(Clone)]
pub struct ApiClientBuilder {
    base_url: String,
    token: String,
    proxy: Option<String>,
    retry: Option<RetryConfig>,
    rate_limit: Option<RateLimitConfig>,
    search_rate_limit: Option<RateLimitConfig>,
    timeout: Duration,
    on_retry: Option<Arc<dyn Fn(RetryInfo) + Send + Sync>>,
}

impl std::fmt::Debug for ApiClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiClientBuilder")
            .field("base_url", &self.base_url)
            .field("token", &"[redacted]")
            .field("proxy", &self.proxy)
            .field("retry", &self.retry)
            .field("rate_limit", &self.rate_limit)
            .field("search_rate_limit", &self.search_rate_limit)
            .field("timeout", &self.timeout)
            .field("on_retry", &self.on_retry.as_ref().map(|_| ".."))
            .finish()
    }
}

impl ApiClientBuilder {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            token: token.into(),
            proxy: None,
            retry: Some(RetryConfig::default()),
            rate_limit: None,
            search_rate_limit: None,
            timeout: Duration::from_secs(30),
            on_retry: None,
        }
    }

    /// Set proxy URL (http, https, socks5).
    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.proxy = Some(proxy_url.into());
        self
    }

    /// Set maximum number of retries.
    pub fn max_retries(mut self, n: u32) -> Self {
        self.retry = Some(RetryConfig {
            max_retries: n,
            ..self.retry.unwrap_or_default()
        });
        self
    }

    /// Set the full retry configuration.
    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry = Some(config);
        self
    }

    /// Disable retries entirely.
    pub fn no_retry(mut self) -> Self {
        self.retry = None;
        self
    }

    /// Set request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the general rate limit (requests per minute).
    pub fn rate_limit(mut self, requests_per_minute: u32) -> Self {
        self.rate_limit = Some(RateLimitConfig {
            requests_per_minute,
        });
        self
    }

    /// Set a separate rate limit for search endpoints (requests per minute).
    pub fn search_rate_limit(mut self, requests_per_minute: u32) -> Self {
        self.search_rate_limit = Some(RateLimitConfig {
            requests_per_minute,
        });
        self
    }

    /// Set the on_retry callback, invoked before each retry attempt.
    pub fn on_retry(mut self, cb: Arc<dyn Fn(RetryInfo) + Send + Sync>) -> Self {
        self.on_retry = Some(cb);
        self
    }

    pub fn build(self) -> Result<ApiClient> {
        // Validate proxy URL if provided
        if let Some(ref proxy_url) = self.proxy {
            let url = reqwest::Url::parse(proxy_url)
                .map_err(|e| Error::Config(format!("invalid proxy URL: {e}")))?;
            match url.scheme() {
                "http" | "https" | "socks5" => {}
                other => return Err(Error::Config(format!("unsupported proxy scheme: {other}"))),
            }
            if url.host().is_none() {
                return Err(Error::Config("proxy URL has no host".to_string()));
            }
        }

        let mut headers = reqwest::header::HeaderMap::new();
        if !self.token.is_empty() {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.token).parse().map_err(|_| {
                    Error::Config(
                        "invalid token: contains characters not allowed in HTTP headers".into(),
                    )
                })?,
            );
        }
        headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());

        let mut builder = Client::builder()
            .timeout(self.timeout)
            .default_headers(headers);

        if let Some(proxy_url) = &self.proxy {
            builder = builder.proxy(
                Proxy::all(proxy_url).map_err(|e| Error::Config(format!("invalid proxy: {e}")))?,
            );
        }

        let http = builder
            .build()
            .map_err(|e| Error::Config(format!("failed to build HTTP client: {e}")))?;

        let rate_limiter = self
            .rate_limit
            .map(|c| Arc::new(RateLimiter::new(c.requests_per_minute)));
        let search_rate_limiter = self
            .search_rate_limit
            .map(|c| Arc::new(RateLimiter::new(c.requests_per_minute)));

        Ok(ApiClient {
            http,
            base_url: self.base_url,
            retry_config: self.retry,
            rate_limiter,
            search_rate_limiter,
            on_retry: self.on_retry,
        })
    }
}

// ── ApiClient ──

/// The core API client. Handles authentication, rate limiting, retries, and proxy.
#[derive(Clone)]
pub struct ApiClient {
    pub(crate) http: Client,
    pub(crate) base_url: String,
    retry_config: Option<RetryConfig>,
    rate_limiter: Option<Arc<RateLimiter>>,
    search_rate_limiter: Option<Arc<RateLimiter>>,
    on_retry: Option<Arc<dyn Fn(RetryInfo) + Send + Sync>>,
}

impl std::fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiClient")
            .field("base_url", &self.base_url)
            .field("retry_config", &self.retry_config)
            .field("has_rate_limiter", &self.rate_limiter.is_some())
            .field(
                "has_search_rate_limiter",
                &self.search_rate_limiter.is_some(),
            )
            .field("has_on_retry", &self.on_retry.is_some())
            .finish()
    }
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Result<Self> {
        ApiClientBuilder::new(base_url, token).build()
    }

    pub fn builder(base_url: impl Into<String>, token: impl Into<String>) -> ApiClientBuilder {
        ApiClientBuilder::new(base_url, token)
    }

    /// Execute a request with auto-retry on 429/502/503/504 and transient network errors.
    ///
    /// If `is_search` is true, the request goes through both the standard and search
    /// rate limiters (when configured).
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
        self.request_inner(method, path, query, body, false).await
    }

    /// Execute a search request — applies both standard and search rate limits.
    pub async fn request_search<Q, R>(
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
        self.request_inner(method, path, query, body, true).await
    }

    async fn request_inner<Q, R>(
        &self,
        method: &str,
        path: &str,
        query: Option<&Q>,
        body: Option<RequestBody>,
        is_search: bool,
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

        let retry_cfg = self.retry_config.as_ref();
        let max_retries = retry_cfg.map_or(0, |c| c.max_retries);
        let base_delay_ms = retry_cfg.map_or(DEFAULT_BASE_DELAY_MS, |c| c.base_delay_ms);
        let max_delay_ms = retry_cfg.map_or(DEFAULT_MAX_DELAY_MS, |c| c.max_delay_ms);

        let mut last_error: Option<Error> = None;

        for attempt in 0..=max_retries {
            // Proactive rate limiting — wait for token before sending
            if let Some(ref limiter) = self.rate_limiter {
                limiter.acquire().await;
            }
            if is_search {
                if let Some(ref search_limiter) = self.search_rate_limiter {
                    search_limiter.acquire().await;
                }
            }

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

            debug!(attempt, method, url = %url, is_search, "sending request");

            let resp = match req.send().await {
                Ok(r) => r,
                Err(e) if attempt < max_retries && is_retryable_transport_error(&e) => {
                    let delay = compute_backoff(base_delay_ms, max_delay_ms, attempt);
                    warn!(attempt, error = %e, "transport error, retrying in {:?}", delay);

                    if let Some(ref cb) = self.on_retry {
                        cb(RetryInfo {
                            attempt: attempt + 1,
                            delay_ms: delay.as_millis() as u64,
                            method: method.to_string(),
                            path: path.to_string(),
                            status: None,
                        });
                    }

                    tokio::time::sleep(delay).await;
                    last_error = Some(Error::Http(e));
                    continue;
                }
                Err(e) => {
                    if attempt > 0 {
                        return Err(Error::RetryExhausted {
                            attempts: attempt + 1,
                            last_error: Box::new(Error::Http(e)),
                        });
                    }
                    return Err(Error::Http(e));
                }
            };

            let status = resp.status();
            if is_retryable_status(status) && attempt < max_retries {
                let retry_after = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(parse_retry_after);

                let delay = retry_after
                    .unwrap_or_else(|| compute_backoff(base_delay_ms, max_delay_ms, attempt));

                warn!(
                    attempt,
                    status = status.as_u16(),
                    "retryable status, waiting {:?}",
                    delay
                );

                if let Some(ref cb) = self.on_retry {
                    cb(RetryInfo {
                        attempt: attempt + 1,
                        delay_ms: delay.as_millis() as u64,
                        method: method.to_string(),
                        path: path.to_string(),
                        status: Some(status.as_u16()),
                    });
                }

                tokio::time::sleep(delay).await;
                last_error = Some(Error::Api {
                    status: status.as_u16(),
                    body: String::new(),
                });
                continue;
            }

            let status_code = status.as_u16();
            let response_text = resp.text().await.map_err(Error::Http)?;

            if status == StatusCode::TOO_MANY_REQUESTS {
                // Exhausted retries on rate limit
                return Err(Error::RetryExhausted {
                    attempts: attempt + 1,
                    last_error: Box::new(Error::RateLimited {
                        attempts: attempt + 1,
                    }),
                });
            }

            if !status.is_success() {
                let message = extract_error_message(&response_text);
                let err = match status_code {
                    401 => Error::Auth { message },
                    403 => Error::Forbidden { message },
                    404 => Error::NotFound { message },
                    _ => Error::Api {
                        status: status_code,
                        body: response_text,
                    },
                };
                if attempt > 0 && is_retryable_status(status) {
                    return Err(Error::RetryExhausted {
                        attempts: attempt + 1,
                        last_error: Box::new(err),
                    });
                }
                return Err(err);
            }

            let parsed: R = serde_json::from_str(&response_text).map_err(Error::Json)?;
            return Ok(parsed);
        }

        // All retries exhausted
        Err(Error::RetryExhausted {
            attempts: max_retries + 1,
            last_error: Box::new(last_error.unwrap_or(Error::RateLimited {
                attempts: max_retries + 1,
            })),
        })
    }
}
