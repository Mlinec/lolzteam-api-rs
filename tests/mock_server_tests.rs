// Mock TCP server tests for retry, rate limiting, and Retry-After behavior.
// These tests spin up a real HTTP server and verify client behavior end-to-end.

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use lolzteam::client::{ApiClient, RetryInfo};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Bind a TcpListener on a random port and return (listener, address).
async fn bind_random() -> (TcpListener, SocketAddr) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    (listener, addr)
}

/// Spin up a one-connection HTTP/1.1 server that calls `handler` for each request.
/// Returns after one connection is served.
async fn serve_one<F, Fut>(listener: TcpListener, handler: F)
where
    F: Fn(Request<Incoming>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Response<Full<Bytes>>> + Send,
{
    let (stream, _) = listener.accept().await.unwrap();
    let io = TokioIo::new(stream);
    let handler = Arc::new(handler);
    let svc = service_fn(move |req| {
        let h = handler.clone();
        async move { Ok::<_, hyper::Error>(h(req).await) }
    });
    let _ = http1::Builder::new()
        .keep_alive(true)
        .serve_connection(io, svc)
        .await;
}

/// Spin up a server that handles exactly N requests on a single connection.
async fn serve_requests<F>(listener: TcpListener, handler: F)
where
    F: Fn(Request<Incoming>) -> Response<Full<Bytes>> + Send + Sync + 'static,
{
    let (stream, _) = listener.accept().await.unwrap();
    let io = TokioIo::new(stream);
    let handler = Arc::new(handler);
    let svc = service_fn(move |req| {
        let h = handler.clone();
        async move { Ok::<_, hyper::Error>(h(req)) }
    });
    let _ = http1::Builder::new()
        .keep_alive(true)
        .serve_connection(io, svc)
        .await;
}

fn json_ok(body: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(body.to_string())))
        .unwrap()
}

fn status_response(code: u16, body: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::from_u16(code).unwrap())
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(body.to_string())))
        .unwrap()
}

fn status_with_retry_after(code: u16, retry_after: &str, body: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::from_u16(code).unwrap())
        .header("content-type", "application/json")
        .header("retry-after", retry_after)
        .body(Full::new(Bytes::from(body.to_string())))
        .unwrap()
}

fn make_client(addr: SocketAddr, max_retries: u32, base_delay_ms: u64) -> ApiClient {
    ApiClient::builder(format!("http://{addr}"), "test-token")
        .max_retries(max_retries)
        .timeout(Duration::from_secs(5))
        .retry_config(lolzteam::client::RetryConfig {
            max_retries,
            base_delay_ms,
            max_delay_ms: 2000,
        })
        .build()
        .unwrap()
}

// ---------------------------------------------------------------------------
// Tests: successful request (200)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_200_returns_json() {
    let (listener, addr) = bind_random().await;

    let server = tokio::spawn(async move {
        serve_requests(listener, |_req| json_ok(r#"{"status":"ok","value":42}"#)).await;
    });

    let client = make_client(addr, 0, 100);
    let result: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();

    assert_eq!(result["status"], "ok");
    assert_eq!(result["value"], 42);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: 429 → retry → 200
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_429_then_200_retries_successfully() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            let n = cnt.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                status_response(429, r#"{"error":"rate limited"}"#)
            } else {
                json_ok(r#"{"status":"ok"}"#)
            }
        })
        .await;
    });

    let client = make_client(addr, 3, 50);
    let result: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();

    assert_eq!(result["status"], "ok");
    assert!(
        counter.load(Ordering::SeqCst) >= 2,
        "should have retried at least once"
    );

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: 502 → retry → 200
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_502_then_200_retries_successfully() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            let n = cnt.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                status_response(502, r#"{"error":"bad gateway"}"#)
            } else {
                json_ok(r#"{"status":"ok"}"#)
            }
        })
        .await;
    });

    let client = make_client(addr, 3, 50);
    let result: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();

    assert_eq!(result["status"], "ok");
    assert!(counter.load(Ordering::SeqCst) >= 2);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: 503 → retry → 200
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_503_then_200_retries_successfully() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            let n = cnt.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                status_response(503, r#"{"error":"service unavailable"}"#)
            } else {
                json_ok(r#"{"status":"ok"}"#)
            }
        })
        .await;
    });

    let client = make_client(addr, 3, 50);
    let result: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();

    assert_eq!(result["status"], "ok");
    assert!(counter.load(Ordering::SeqCst) >= 2);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: 504 → retry → 200
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_504_then_200_retries_successfully() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            let n = cnt.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                status_response(504, r#"{"error":"gateway timeout"}"#)
            } else {
                json_ok(r#"{"status":"ok"}"#)
            }
        })
        .await;
    });

    let client = make_client(addr, 3, 50);
    let result: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();

    assert_eq!(result["status"], "ok");
    assert!(counter.load(Ordering::SeqCst) >= 2);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: Retry-After header (seconds) is respected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_retry_after_seconds_is_respected() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            let n = cnt.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                status_with_retry_after(429, "1", r#"{"error":"rate limited"}"#)
            } else {
                json_ok(r#"{"status":"ok"}"#)
            }
        })
        .await;
    });

    let client = make_client(addr, 3, 50);
    let start = Instant::now();
    let result: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();
    let elapsed = start.elapsed();

    assert_eq!(result["status"], "ok");
    // Should have waited at least ~1 second due to Retry-After: 1
    assert!(
        elapsed >= Duration::from_millis(800),
        "expected >=800ms wait for Retry-After:1, got {:?}",
        elapsed
    );

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: on_retry callback receives correct info on HTTP error
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_on_retry_callback_with_http_status() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            let n = cnt.fetch_add(1, Ordering::SeqCst);
            if n < 2 {
                status_response(502, r#"{"error":"bad gateway"}"#)
            } else {
                json_ok(r#"{"status":"ok"}"#)
            }
        })
        .await;
    });

    let retry_calls = Arc::new(AtomicU32::new(0));
    let retry_statuses: Arc<tokio::sync::Mutex<Vec<Option<u16>>>> =
        Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let rc = retry_calls.clone();
    let rs = retry_statuses.clone();

    let client = ApiClient::builder(format!("http://{addr}"), "test-token")
        .retry_config(lolzteam::client::RetryConfig {
            max_retries: 5,
            base_delay_ms: 50,
            max_delay_ms: 500,
        })
        .timeout(Duration::from_secs(5))
        .on_retry(Arc::new(move |info: RetryInfo| {
            rc.fetch_add(1, Ordering::SeqCst);
            let rs2 = rs.clone();
            // Store the status (we can't await here, so use try_lock)
            if let Ok(mut vec) = rs2.try_lock() {
                vec.push(info.status);
            };
        }))
        .build()
        .unwrap();

    let result: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();

    assert_eq!(result["status"], "ok");
    assert_eq!(
        retry_calls.load(Ordering::SeqCst),
        2,
        "on_retry should be called twice"
    );

    let statuses = retry_statuses.lock().await;
    assert_eq!(statuses.len(), 2);
    assert_eq!(statuses[0], Some(502));
    assert_eq!(statuses[1], Some(502));

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: all retries exhausted → RetryExhausted error
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_all_retries_exhausted() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            cnt.fetch_add(1, Ordering::SeqCst);
            status_response(502, r#"{"error":"always failing"}"#)
        })
        .await;
    });

    let client = make_client(addr, 2, 50); // max 2 retries = 3 total attempts
    let result = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    match &err {
        lolzteam::Error::RetryExhausted { attempts, .. } => {
            assert_eq!(*attempts, 3, "should have made 3 attempts (1 + 2 retries)");
        }
        other => panic!("expected RetryExhausted, got: {other:?}"),
    }
    // Verify the server received all 3 requests
    assert_eq!(counter.load(Ordering::SeqCst), 3);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: no_retry → fail immediately without retrying
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_no_retry_fails_immediately() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            cnt.fetch_add(1, Ordering::SeqCst);
            status_response(502, r#"{"error":"bad gateway"}"#)
        })
        .await;
    });

    let client = ApiClient::builder(format!("http://{addr}"), "test-token")
        .no_retry()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let result = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await;

    assert!(result.is_err());
    // Should have made only 1 request (no retries)
    assert_eq!(
        counter.load(Ordering::SeqCst),
        1,
        "no_retry should not retry"
    );

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: 401 → Auth error, NOT retried
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_401_returns_auth_error_no_retry() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            cnt.fetch_add(1, Ordering::SeqCst);
            status_response(401, r#"{"error":{"message":"unauthorized"}}"#)
        })
        .await;
    });

    let client = make_client(addr, 3, 50);
    let result = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        lolzteam::Error::Auth { message } => {
            assert_eq!(message, "unauthorized");
        }
        other => panic!("expected Auth error, got: {other:?}"),
    }
    // Should not retry on 401
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: 403 → Forbidden error, NOT retried
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_403_returns_forbidden_error_no_retry() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            cnt.fetch_add(1, Ordering::SeqCst);
            status_response(403, r#"{"error":{"message":"forbidden"}}"#)
        })
        .await;
    });

    let client = make_client(addr, 3, 50);
    let result = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        lolzteam::Error::Forbidden { message } => {
            assert_eq!(message, "forbidden");
        }
        other => panic!("expected Forbidden error, got: {other:?}"),
    }
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: 404 → NotFound error, NOT retried
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_404_returns_not_found_error_no_retry() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            cnt.fetch_add(1, Ordering::SeqCst);
            status_response(404, r#"{"error":{"message":"not found"}}"#)
        })
        .await;
    });

    let client = make_client(addr, 3, 50);
    let result = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        lolzteam::Error::NotFound { message } => {
            assert_eq!(message, "not found");
        }
        other => panic!("expected NotFound error, got: {other:?}"),
    }
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: Authorization header is sent correctly
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_authorization_header_is_sent() {
    let (listener, addr) = bind_random().await;
    let auth_header: Arc<tokio::sync::Mutex<Option<String>>> =
        Arc::new(tokio::sync::Mutex::new(None));
    let ah = auth_header.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |req| {
            let val = req
                .headers()
                .get("authorization")
                .map(|v| v.to_str().unwrap().to_string());
            if let Ok(mut guard) = ah.try_lock() {
                *guard = val;
            }
            json_ok(r#"{"ok":true}"#)
        })
        .await;
    });

    let client = ApiClient::builder(format!("http://{addr}"), "my-secret-token")
        .no_retry()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let _: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();

    let header = auth_header.lock().await;
    assert_eq!(header.as_deref(), Some("Bearer my-secret-token"));

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: Multiple 429s → success after several retries
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_multiple_429_then_success() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            let n = cnt.fetch_add(1, Ordering::SeqCst);
            if n < 3 {
                status_response(429, r#"{"error":"rate limited"}"#)
            } else {
                json_ok(r#"{"status":"ok","attempt":4}"#)
            }
        })
        .await;
    });

    let client = make_client(addr, 5, 50);
    let result: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();

    assert_eq!(result["status"], "ok");
    assert_eq!(counter.load(Ordering::SeqCst), 4);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: Query parameters are forwarded correctly
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_query_params_forwarded() {
    let (listener, addr) = bind_random().await;
    let captured_uri: Arc<tokio::sync::Mutex<Option<String>>> =
        Arc::new(tokio::sync::Mutex::new(None));
    let cu = captured_uri.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |req| {
            if let Ok(mut guard) = cu.try_lock() {
                *guard = Some(req.uri().to_string());
            }
            json_ok(r#"{"ok":true}"#)
        })
        .await;
    });

    let client = ApiClient::builder(format!("http://{addr}"), "token")
        .no_retry()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let query = vec![("page", "1"), ("limit", "10")];
    let _: serde_json::Value = client
        .request("get", "/items", Some(&query), None)
        .await
        .unwrap();

    let uri = captured_uri.lock().await;
    let uri_str = uri.as_deref().unwrap();
    assert!(uri_str.contains("page=1"), "URI: {uri_str}");
    assert!(uri_str.contains("limit=10"), "URI: {uri_str}");

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: POST body is sent correctly
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_post_json_body_sent() {
    use lolzteam::client::RequestBody;

    let (listener, addr) = bind_random().await;
    let captured_body: Arc<tokio::sync::Mutex<Option<String>>> =
        Arc::new(tokio::sync::Mutex::new(None));
    let cb = captured_body.clone();

    let server = tokio::spawn(async move {
        serve_one(listener, move |req| {
            let cb2 = cb.clone();
            async move {
                let body_bytes = http_body_util::BodyExt::collect(req.into_body())
                    .await
                    .unwrap()
                    .to_bytes();
                let body_str = String::from_utf8_lossy(&body_bytes).to_string();
                if let Ok(mut guard) = cb2.try_lock() {
                    *guard = Some(body_str);
                }
                json_ok(r#"{"ok":true}"#)
            }
        })
        .await;
    });

    let client = ApiClient::builder(format!("http://{addr}"), "token")
        .no_retry()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let body = RequestBody::Json(serde_json::json!({"key": "value", "num": 42}));
    let _: serde_json::Value = client
        .request::<(), serde_json::Value>("post", "/create", None, Some(body))
        .await
        .unwrap();

    let body = captured_body.lock().await;
    let body_str = body.as_deref().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(body_str).unwrap();
    assert_eq!(parsed["key"], "value");
    assert_eq!(parsed["num"], 42);

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: Rate limiter doesn't block when disabled
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_no_rate_limit_fast() {
    let (listener, addr) = bind_random().await;

    let server = tokio::spawn(async move {
        serve_requests(listener, |_req| json_ok(r#"{"ok":true}"#)).await;
    });

    // No rate limit configured → should be fast
    let client = ApiClient::builder(format!("http://{addr}"), "token")
        .no_retry()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let start = Instant::now();
    for _ in 0..5 {
        let _: serde_json::Value = client
            .request::<(), serde_json::Value>("get", "/test", None, None)
            .await
            .unwrap();
    }
    let elapsed = start.elapsed();

    // 5 requests without rate limiter should be very fast
    assert!(
        elapsed < Duration::from_secs(2),
        "5 requests without rate limiter took too long: {:?}",
        elapsed
    );

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: search request goes through search rate limiter
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_search_rate_limiter_applied() {
    let (listener, addr) = bind_random().await;

    let server = tokio::spawn(async move {
        serve_requests(listener, |_req| json_ok(r#"{"ok":true}"#)).await;
    });

    let client = ApiClient::builder(format!("http://{addr}"), "token")
        .no_retry()
        .timeout(Duration::from_secs(5))
        .rate_limit(6000) // high general limit
        .search_rate_limit(6000) // high search limit too
        .build()
        .unwrap();

    let debug = format!("{:?}", client);
    assert!(debug.contains("has_search_rate_limiter: true"));

    let _: serde_json::Value = client
        .request_search::<(), serde_json::Value>("get", "/search", None, None)
        .await
        .unwrap();

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: exponential backoff timing
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_backoff_increases_with_attempts() {
    let (listener, addr) = bind_random().await;
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |_req| {
            let n = cnt.fetch_add(1, Ordering::SeqCst);
            if n < 3 {
                status_response(502, r#"{"error":"fail"}"#)
            } else {
                json_ok(r#"{"ok":true}"#)
            }
        })
        .await;
    });

    let client = make_client(addr, 5, 100); // base_delay=100ms
    let start = Instant::now();
    let _: serde_json::Value = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await
        .unwrap();
    let elapsed = start.elapsed();

    // 3 retries with base_delay=100ms: ~100ms + ~200ms + ~400ms = ~700ms minimum
    // With jitter it could be a bit less, but should be noticeable
    assert!(
        elapsed >= Duration::from_millis(200),
        "backoff should have caused delays, got {:?}",
        elapsed
    );

    let _ = server.await;
}

// ---------------------------------------------------------------------------
// Tests: HTTP methods (POST, PUT, DELETE, PATCH)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mock_post_method() {
    let (listener, addr) = bind_random().await;
    let captured_method: Arc<tokio::sync::Mutex<Option<String>>> =
        Arc::new(tokio::sync::Mutex::new(None));
    let cm = captured_method.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |req| {
            if let Ok(mut guard) = cm.try_lock() {
                *guard = Some(req.method().to_string());
            }
            json_ok(r#"{"ok":true}"#)
        })
        .await;
    });

    let client = ApiClient::builder(format!("http://{addr}"), "token")
        .no_retry()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let _: serde_json::Value = client
        .request::<(), serde_json::Value>("post", "/test", None, None)
        .await
        .unwrap();

    let method = captured_method.lock().await;
    assert_eq!(method.as_deref(), Some("POST"));

    let _ = server.await;
}

#[tokio::test]
async fn mock_delete_method() {
    let (listener, addr) = bind_random().await;
    let captured_method: Arc<tokio::sync::Mutex<Option<String>>> =
        Arc::new(tokio::sync::Mutex::new(None));
    let cm = captured_method.clone();

    let server = tokio::spawn(async move {
        serve_requests(listener, move |req| {
            if let Ok(mut guard) = cm.try_lock() {
                *guard = Some(req.method().to_string());
            }
            json_ok(r#"{"ok":true}"#)
        })
        .await;
    });

    let client = ApiClient::builder(format!("http://{addr}"), "token")
        .no_retry()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let _: serde_json::Value = client
        .request::<(), serde_json::Value>("delete", "/test", None, None)
        .await
        .unwrap();

    let method = captured_method.lock().await;
    assert_eq!(method.as_deref(), Some("DELETE"));

    let _ = server.await;
}
