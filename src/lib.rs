//! # lolzteam
//!
//! Rust-клиент для LOLZTEAM Forum & Market API.
//! Методы и типы сгенерированы из OpenAPI-схем.
//!
//! ## Быстрый старт
//!
//! ```rust,no_run
//! use lolzteam::LolzteamClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LolzteamClient::new("YOUR_TOKEN")?;
//!
//!     let user = client.forum().users_get(1, None).await?;
//!     println!("{:?}", user);
//!
//!     let threads = client.forum().threads_list(Default::default()).await?;
//!     println!("{:?}", threads);
//!
//!     let items = client.market().category_steam(Default::default()).await?;
//!     println!("{:?}", items);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Прокси
//!
//!
//! ```rust,no_run
//! use lolzteam::LolzteamClient;
//!
//! let client = LolzteamClient::builder("YOUR_TOKEN")
//!     .forum_proxy("socks5://127.0.0.1:1080")
//!     .market_proxy("http://user:pass@proxy.example.com:8080")
//!     .max_retries(3)
//!     .build()
//!     .unwrap();
//! ```

pub mod client;
pub mod error;
pub mod forum;
pub mod market;
pub mod models;

pub use client::{
    ApiClient, ApiClientBuilder, MultipartFile, MultipartForm, RateLimitConfig, RateLimiter,
    RequestBody, RetryConfig, RetryInfo,
};
pub use error::Error;

use std::sync::Arc;
use std::time::Duration;

pub const FORUM_BASE_URL: &str = "https://prod-api.lolz.live";
pub const MARKET_BASE_URL: &str = "https://prod-api.lzt.market";

/// Default Forum rate limit: 300 requests per minute.
pub const FORUM_RATE_LIMIT: u32 = 300;
/// Default Market rate limit: 120 requests per minute.
pub const MARKET_RATE_LIMIT: u32 = 120;
/// Default Market search rate limit: 20 requests per minute.
pub const MARKET_SEARCH_RATE_LIMIT: u32 = 20;

pub struct LolzteamClient {
    forum_client: ApiClient,
    market_client: ApiClient,
}

pub struct LolzteamClientBuilder {
    token: String,
    forum_base: String,
    market_base: String,
    forum_proxy: Option<String>,
    market_proxy: Option<String>,
    max_retries: u32,
    base_delay_ms: u64,
    max_delay_ms: u64,
    timeout: Duration,
    forum_rate_limit: Option<u32>,
    market_rate_limit: Option<u32>,
    market_search_rate_limit: Option<u32>,
    on_retry: Option<Arc<dyn Fn(RetryInfo) + Send + Sync>>,
}

impl LolzteamClientBuilder {
    /// Set a separate proxy for the Forum API.
    pub fn forum_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.forum_proxy = Some(proxy.into());
        self
    }

    /// Set a separate proxy for the Market API.
    pub fn market_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.market_proxy = Some(proxy.into());
        self
    }

    /// Set the same proxy for both Forum and Market APIs.
    pub fn proxy(self, proxy: impl Into<String>) -> Self {
        let p = proxy.into();
        self.forum_proxy(p.clone()).market_proxy(p)
    }

    /// Set the maximum number of retries for transient errors.
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    /// Set the base delay for exponential backoff (in milliseconds).
    pub fn base_delay_ms(mut self, ms: u64) -> Self {
        self.base_delay_ms = ms;
        self
    }

    /// Set the maximum delay for exponential backoff (in milliseconds).
    pub fn max_delay_ms(mut self, ms: u64) -> Self {
        self.max_delay_ms = ms;
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the Forum API base URL.
    pub fn forum_base_url(mut self, url: impl Into<String>) -> Self {
        self.forum_base = url.into();
        self
    }

    /// Set the Market API base URL.
    pub fn market_base_url(mut self, url: impl Into<String>) -> Self {
        self.market_base = url.into();
        self
    }

    /// Override the Forum rate limit (requests per minute). Defaults to 300.
    pub fn forum_rate_limit(mut self, rpm: u32) -> Self {
        self.forum_rate_limit = Some(rpm);
        self
    }

    /// Override the Market rate limit (requests per minute). Defaults to 120.
    pub fn market_rate_limit(mut self, rpm: u32) -> Self {
        self.market_rate_limit = Some(rpm);
        self
    }

    /// Override the Market search rate limit (requests per minute). Defaults to 20.
    pub fn market_search_rate_limit(mut self, rpm: u32) -> Self {
        self.market_search_rate_limit = Some(rpm);
        self
    }

    /// Disable rate limiting entirely.
    pub fn no_rate_limit(mut self) -> Self {
        self.forum_rate_limit = Some(0);
        self.market_rate_limit = Some(0);
        self.market_search_rate_limit = Some(0);
        self
    }

    /// Set the on_retry callback, invoked before each retry attempt.
    pub fn on_retry(mut self, cb: Arc<dyn Fn(RetryInfo) + Send + Sync>) -> Self {
        self.on_retry = Some(cb);
        self
    }

    pub fn build(self) -> error::Result<LolzteamClient> {
        let retry_cfg = RetryConfig {
            max_retries: self.max_retries,
            base_delay_ms: self.base_delay_ms,
            max_delay_ms: self.max_delay_ms,
        };

        let forum_client = Self::make_client(
            &self.forum_base,
            &self.token,
            &self.forum_proxy,
            retry_cfg.clone(),
            self.timeout,
            self.forum_rate_limit.unwrap_or(FORUM_RATE_LIMIT),
            None, // Forum doesn't have separate search rate limit
            self.on_retry.clone(),
        )?;
        let market_client = Self::make_client(
            &self.market_base,
            &self.token,
            &self.market_proxy,
            retry_cfg,
            self.timeout,
            self.market_rate_limit.unwrap_or(MARKET_RATE_LIMIT),
            Some(
                self.market_search_rate_limit
                    .unwrap_or(MARKET_SEARCH_RATE_LIMIT),
            ),
            self.on_retry,
        )?;
        Ok(LolzteamClient {
            forum_client,
            market_client,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn make_client(
        base_url: &str,
        token: &str,
        proxy: &Option<String>,
        retry_cfg: RetryConfig,
        timeout: Duration,
        rate_limit_rpm: u32,
        search_rate_limit_rpm: Option<u32>,
        on_retry: Option<Arc<dyn Fn(RetryInfo) + Send + Sync>>,
    ) -> error::Result<ApiClient> {
        let mut b = ApiClient::builder(base_url, token)
            .retry_config(retry_cfg)
            .timeout(timeout);
        if let Some(ref p) = proxy {
            b = b.proxy(p);
        }
        if rate_limit_rpm > 0 {
            b = b.rate_limit(rate_limit_rpm);
        }
        if let Some(srpm) = search_rate_limit_rpm {
            if srpm > 0 {
                b = b.search_rate_limit(srpm);
            }
        }
        if let Some(cb) = on_retry {
            b = b.on_retry(cb);
        }
        b.build()
    }
}

impl LolzteamClient {
    /// Create a new client with default configuration.
    ///
    /// Defaults: 5 retries, 1s base delay, 60s max delay, 30s timeout,
    /// Forum: 300 req/min, Market: 120 req/min + 20 search/min.
    pub fn new(token: impl Into<String>) -> error::Result<Self> {
        Self::builder(token).build()
    }

    /// Create a builder for custom configuration.
    pub fn builder(token: impl Into<String>) -> LolzteamClientBuilder {
        LolzteamClientBuilder {
            token: token.into(),
            forum_base: FORUM_BASE_URL.to_string(),
            market_base: MARKET_BASE_URL.to_string(),
            forum_proxy: None,
            market_proxy: None,
            max_retries: 5,
            base_delay_ms: 1000,
            max_delay_ms: 60_000,
            timeout: Duration::from_secs(30),
            forum_rate_limit: None,
            market_rate_limit: None,
            market_search_rate_limit: None,
            on_retry: None,
        }
    }

    /// Access the Forum API.
    pub fn forum(&self) -> forum::ForumApi {
        forum::ForumApi::new(self.forum_client.clone())
    }

    /// Access the Market API.
    pub fn market(&self) -> market::MarketApi {
        market::MarketApi::new(self.market_client.clone())
    }
}
