use lolzteam::{Error, LolzteamClient};

#[tokio::test]
async fn retry_multipart_regression() {
    let client = LolzteamClient::builder("token")
        .forum_base_url("http://127.0.0.1:1")
        .market_base_url("http://127.0.0.1:1")
        .max_retries(3)
        .timeout(std::time::Duration::from_millis(200))
        .no_rate_limit()
        .build()
        .unwrap();

    let err = client
        .forum()
        .o_auth_token(lolzteam::forum::types::ForumOAuthTokenParams {
            client_id: "1".into(),
            client_secret: "s".into(),
            grant_type: "g".into(),
            code: None,
            password: None,
            redirect_uri: None,
            refresh_token: None,
            scope: None,
            username: None,
        })
        .await
        .expect_err("expected request failure against closed port");

    match err {
        Error::Http(_) | Error::RetryExhausted { .. } => {}
        other => panic!("unexpected error: {other:?}"),
    }
}
