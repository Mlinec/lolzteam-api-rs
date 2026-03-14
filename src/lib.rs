//! # lolzteam
//!
//! Rust-клиент для LOLZTEAM Forum & Market API.
//! Методы и типы сгенерированы из OpenAPI-спеки.
//!
//! ## Быстрый старт
//!
//! ```rust,no_run
//! use lolzteam::LolzteamClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LolzteamClient::new("YOUR_TOKEN");
//!
//!     let user = client.forum().users_get(1, None).await?;
//!     println!("{:?}", user);
//!
//!     let cats = client.market().category_list(None).await?;
//!     println!("{:?}", cats);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Прокси
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

pub use client::{ApiClient, ApiClientBuilder};
pub use error::Error;

use std::time::Duration;

pub const FORUM_BASE_URL: &str = "https://prod-api.lolz.live";
pub const MARKET_BASE_URL: &str = "https://prod-api.lzt.market";

pub struct LolzteamClient {
    token: String,
    forum_base: String,
    market_base: String,
    forum_proxy: Option<String>,
    market_proxy: Option<String>,
    max_retries: u32,
    timeout: Duration,
}

pub struct LolzteamClientBuilder {
    inner: LolzteamClient,
}

impl LolzteamClientBuilder {
    pub fn forum_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.inner.forum_proxy = Some(proxy.into());
        self
    }

    pub fn market_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.inner.market_proxy = Some(proxy.into());
        self
    }

    pub fn proxy(self, proxy: impl Into<String>) -> Self {
        let p = proxy.into();
        self.forum_proxy(p.clone()).market_proxy(p)
    }

    pub fn max_retries(mut self, n: u32) -> Self {
        self.inner.max_retries = n;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.inner.timeout = timeout;
        self
    }

    pub fn forum_base_url(mut self, url: impl Into<String>) -> Self {
        self.inner.forum_base = url.into();
        self
    }

    pub fn market_base_url(mut self, url: impl Into<String>) -> Self {
        self.inner.market_base = url.into();
        self
    }

    pub fn build(self) -> error::Result<LolzteamClient> {
        let _ = self.inner.make_forum_client()?;
        let _ = self.inner.make_market_client()?;
        Ok(self.inner)
    }
}

impl LolzteamClient {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            forum_base: FORUM_BASE_URL.to_string(),
            market_base: MARKET_BASE_URL.to_string(),
            forum_proxy: None,
            market_proxy: None,
            max_retries: 5,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn builder(token: impl Into<String>) -> LolzteamClientBuilder {
        LolzteamClientBuilder {
            inner: Self::new(token),
        }
    }

    fn make_forum_client(&self) -> error::Result<ApiClient> {
        let mut b = ApiClient::builder(&self.forum_base, &self.token)
            .max_retries(self.max_retries)
            .timeout(self.timeout);
        if let Some(ref p) = self.forum_proxy {
            b = b.proxy(p);
        }
        b.build()
    }

    fn make_market_client(&self) -> error::Result<ApiClient> {
        let mut b = ApiClient::builder(&self.market_base, &self.token)
            .max_retries(self.max_retries)
            .timeout(self.timeout);
        if let Some(ref p) = self.market_proxy {
            b = b.proxy(p);
        }
        b.build()
    }

    pub fn forum(&self) -> forum::ForumApi {
        forum::ForumApi::new(self.make_forum_client().expect("forum client already validated"))
    }

    pub fn market(&self) -> market::MarketApi {
        market::MarketApi::new(self.make_market_client().expect("market client already validated"))
    }
}
