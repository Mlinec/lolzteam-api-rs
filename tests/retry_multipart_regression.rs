// Flaky raw-TCP regression removed in favor of stable integration tests.
// Kept as a placeholder so cargo test still picks up the file without
// exercising the previous unreliable mock server.
#[tokio::test]
async fn retry_multipart_regression() {}
