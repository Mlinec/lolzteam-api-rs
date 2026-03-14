pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error {status}: {body}")]
    Api { status: u16, body: String },

    #[error("rate limited after {attempts} attempts")]
    RateLimited { attempts: u32 },
}
