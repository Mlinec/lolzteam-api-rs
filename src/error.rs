pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error {status}: {body}")]
    Api { status: u16, body: String },

    #[error("auth error: {message}")]
    Auth { message: String },

    #[error("forbidden: {message}")]
    Forbidden { message: String },

    #[error("not found: {message}")]
    NotFound { message: String },

    #[error("rate limited after {attempts} attempts")]
    RateLimited { attempts: u32 },

    #[error("retry exhausted after {attempts} attempts")]
    RetryExhausted {
        attempts: u32,
        last_error: Box<Error>,
    },

    #[error("configuration error: {0}")]
    Config(String),
}

impl Error {
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::Auth { .. } => Some(401),
            Error::Forbidden { .. } => Some(403),
            Error::NotFound { .. } => Some(404),
            Error::RateLimited { .. } => Some(429),
            Error::Api { status, .. } => Some(*status),
            Error::RetryExhausted { last_error, .. } => last_error.status_code(),
            _ => None,
        }
    }

    /// Returns `true` if this is a retryable error (429/502/503/504 or transient network).
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Api { status, .. } => matches!(*status, 429 | 502 | 503 | 504),
            Error::RateLimited { .. } => true,
            Error::Http(e) => e.is_timeout() || e.is_connect(),
            _ => false,
        }
    }

    /// Returns `true` if this is a rate limit error (429).
    pub fn is_rate_limit(&self) -> bool {
        matches!(
            self,
            Error::RateLimited { .. } | Error::Api { status: 429, .. }
        )
    }

    /// Returns `true` if this is an authentication error (401).
    pub fn is_auth(&self) -> bool {
        matches!(self, Error::Auth { .. })
    }

    /// Returns `true` if this is a not found error (404).
    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::NotFound { .. })
    }
}
