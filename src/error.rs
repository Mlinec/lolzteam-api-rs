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
}

impl Error {
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::Auth { .. } => Some(401),
            Error::Forbidden { .. } => Some(403),
            Error::NotFound { .. } => Some(404),
            Error::RateLimited { .. } => Some(429),
            Error::Api { status, .. } => Some(*status),
            _ => None,
        }
    }
}
