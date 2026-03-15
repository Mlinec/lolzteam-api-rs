pub mod methods;
pub mod types;

use crate::client::ApiClient;

/// Forum API wrapper.
pub struct ForumApi {
    pub(crate) client: ApiClient,
}

impl ForumApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }
}
