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

pub use client::{ApiClient, ApiClientBuilder, MultipartFile, MultipartForm, RequestBody};
pub use error::Error;

use std::time::Duration;

pub const FORUM_BASE_URL: &str = "https://prod-api.lolz.live";
pub const MARKET_BASE_URL: &str = "https://prod-api.lzt.market";

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
    timeout: Duration,
}

impl LolzteamClientBuilder {
    pub fn forum_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.forum_proxy = Some(proxy.into());
        self
    }

    pub fn market_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.market_proxy = Some(proxy.into());
        self
    }

    pub fn proxy(self, proxy: impl Into<String>) -> Self {
        let p = proxy.into();
        self.forum_proxy(p.clone()).market_proxy(p)
    }

    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn forum_base_url(mut self, url: impl Into<String>) -> Self {
        self.forum_base = url.into();
        self
    }

    pub fn market_base_url(mut self, url: impl Into<String>) -> Self {
        self.market_base = url.into();
        self
    }

    pub fn build(self) -> error::Result<LolzteamClient> {
        let forum_client = Self::make_client(
            &self.forum_base,
            &self.token,
            &self.forum_proxy,
            self.max_retries,
            self.timeout,
        )?;
        let market_client = Self::make_client(
            &self.market_base,
            &self.token,
            &self.market_proxy,
            self.max_retries,
            self.timeout,
        )?;
        Ok(LolzteamClient {
            forum_client,
            market_client,
        })
    }

    fn make_client(
        base_url: &str,
        token: &str,
        proxy: &Option<String>,
        max_retries: u32,
        timeout: Duration,
    ) -> error::Result<ApiClient> {
        let mut b = ApiClient::builder(base_url, token)
            .max_retries(max_retries)
            .timeout(timeout);
        if let Some(ref p) = proxy {
            b = b.proxy(p);
        }
        b.build()
    }
}

impl LolzteamClient {
    pub fn new(token: impl Into<String>) -> error::Result<Self> {
        let token = token.into();
        let forum_client = ApiClient::builder(FORUM_BASE_URL, &token).build()?;
        let market_client = ApiClient::builder(MARKET_BASE_URL, &token).build()?;
        Ok(Self {
            forum_client,
            market_client,
        })
    }

    pub fn builder(token: impl Into<String>) -> LolzteamClientBuilder {
        LolzteamClientBuilder {
            token: token.into(),
            forum_base: FORUM_BASE_URL.to_string(),
            market_base: MARKET_BASE_URL.to_string(),
            forum_proxy: None,
            market_proxy: None,
            max_retries: 5,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn forum(&self) -> forum::ForumApi {
        forum::ForumApi::new(self.forum_client.clone())
    }

    pub fn market(&self) -> market::MarketApi {
        market::MarketApi::new(self.market_client.clone())
    }
}
