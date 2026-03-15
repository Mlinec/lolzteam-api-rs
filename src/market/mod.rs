pub mod methods;
pub mod types;

use crate::client::ApiClient;

/// Market API wrapper.
pub struct MarketApi {
    pub(crate) client: ApiClient,
}

impl MarketApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }
}
