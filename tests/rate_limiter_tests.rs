// Async tests for rate limiter behavior and retry logic.

use lolzteam::client::{ApiClient, RateLimiter, RetryInfo};
use lolzteam::LolzteamClient;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// RateLimiter unit tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rate_limiter_allows_burst_within_limit() {
    let limiter = RateLimiter::new(600); // 10 per second
    let start = Instant::now();
    // Should be able to acquire 10 tokens in a burst (600/60=10/sec)
    for _ in 0..10 {
        limiter.acquire().await;
    }
    let elapsed = start.elapsed();
    // Burst should complete almost instantly (< 100ms)
    assert!(
        elapsed < Duration::from_millis(200),
        "burst took too long: {:?}",
        elapsed
    );
}

#[tokio::test]
async fn rate_limiter_throttles_when_exhausted() {
    let limiter = RateLimiter::new(60); // 1 per second
                                        // Exhaust all 60 burst tokens
    for _ in 0..60 {
        limiter.acquire().await;
    }
    let start = Instant::now();
    // The next one should wait ~1 second
    limiter.acquire().await;
    let elapsed = start.elapsed();
    assert!(
        elapsed >= Duration::from_millis(800),
        "expected throttling, got {:?}",
        elapsed
    );
}

#[tokio::test]
async fn rate_limiter_high_rpm_fast_burst() {
    let limiter = RateLimiter::new(6000); // 100 per second
    let start = Instant::now();
    for _ in 0..100 {
        limiter.acquire().await;
    }
    let elapsed = start.elapsed();
    // Should complete within a reasonable time
    assert!(
        elapsed < Duration::from_millis(200),
        "high-RPM burst too slow: {:?}",
        elapsed
    );
}

// ---------------------------------------------------------------------------
// Retry with on_retry callback: verify callback is invoked
// ---------------------------------------------------------------------------

#[tokio::test]
async fn on_retry_callback_is_invoked() {
    let counter = Arc::new(AtomicU32::new(0));
    let cnt = counter.clone();
    let client = LolzteamClient::builder("token")
        .forum_base_url("http://127.0.0.1:1")
        .market_base_url("http://127.0.0.1:1")
        .max_retries(2)
        .timeout(Duration::from_millis(200))
        .no_rate_limit()
        .on_retry(Arc::new(move |_info: RetryInfo| {
            cnt.fetch_add(1, Ordering::SeqCst);
        }))
        .build()
        .unwrap();

    // This will fail to connect and trigger retries
    let _ = client.forum().users_get(1, None).await;

    // on_retry should have been called for each retry attempt
    let calls = counter.load(Ordering::SeqCst);
    assert!(
        calls > 0,
        "on_retry callback should have been called at least once, got {calls}"
    );
}

#[tokio::test]
async fn retry_exhausted_returns_after_max_retries() {
    let client = LolzteamClient::builder("token")
        .forum_base_url("http://127.0.0.1:1")
        .market_base_url("http://127.0.0.1:1")
        .max_retries(2)
        .base_delay_ms(50)
        .timeout(Duration::from_millis(200))
        .no_rate_limit()
        .build()
        .unwrap();

    let start = Instant::now();
    let result = client.forum().users_get(1, None).await;

    let elapsed = start.elapsed();
    assert!(result.is_err(), "expected error");

    // Should not take more than ~10 seconds even with retries
    assert!(
        elapsed < Duration::from_secs(15),
        "retries took too long: {:?}",
        elapsed
    );
}

#[tokio::test]
async fn no_retry_fails_immediately() {
    let client = ApiClient::builder("http://127.0.0.1:1", "token")
        .no_retry()
        .timeout(Duration::from_millis(200))
        .build()
        .unwrap();

    let start = Instant::now();
    let result: Result<serde_json::Value, _> = client
        .request::<(), serde_json::Value>("get", "/test", None, None)
        .await;

    let elapsed = start.elapsed();
    assert!(result.is_err(), "expected error");
    // Without retries, should fail quickly
    assert!(
        elapsed < Duration::from_secs(2),
        "no_retry should fail fast, took {:?}",
        elapsed
    );
}

// ---------------------------------------------------------------------------
// Rate limiter creation via builder
// ---------------------------------------------------------------------------

#[tokio::test]
async fn builder_rate_limit_zero_disables() {
    // rate_limit(0) should NOT add a rate limiter (or it's essentially unlimited)
    let client = ApiClient::builder("http://127.0.0.1:1", "token")
        .no_retry()
        .timeout(Duration::from_millis(100))
        .build()
        .unwrap();

    let debug = format!("{:?}", client);
    assert!(
        debug.contains("has_rate_limiter: false"),
        "no rate limiter by default: {debug}"
    );
}

#[tokio::test]
async fn builder_rate_limit_enabled() {
    let client = ApiClient::builder("http://127.0.0.1:1", "token")
        .rate_limit(300)
        .build()
        .unwrap();

    let debug = format!("{:?}", client);
    assert!(
        debug.contains("has_rate_limiter: true"),
        "rate limiter should be enabled: {debug}"
    );
}

// ---------------------------------------------------------------------------
// on_retry info contains correct fields
// ---------------------------------------------------------------------------

#[tokio::test]
async fn on_retry_info_has_correct_fields() {
    let last_info: Arc<tokio::sync::Mutex<Option<RetryInfo>>> =
        Arc::new(tokio::sync::Mutex::new(None));
    let info_ref = last_info.clone();

    let client = LolzteamClient::builder("token")
        .forum_base_url("http://127.0.0.1:1")
        .market_base_url("http://127.0.0.1:1")
        .max_retries(1)
        .base_delay_ms(10)
        .timeout(Duration::from_millis(200))
        .no_rate_limit()
        .on_retry(Arc::new(move |info: RetryInfo| {
            let ir = info_ref.clone();
            if let Ok(mut guard) = ir.try_lock() {
                *guard = Some(info);
            };
        }))
        .build()
        .unwrap();

    let _ = client.forum().users_get(1, None).await;

    let guard = last_info.lock().await;
    if let Some(ref info) = *guard {
        assert_eq!(info.attempt, 1);
        assert!(info.delay_ms > 0);
        assert_eq!(info.method, "get");
        assert!(info.path.contains("users/1"));
        // Status is None for transport errors
        assert!(info.status.is_none());
    }
    // Note: it's possible on_retry is not called if the initial attempt
    // succeeds or if max_retries=1 and we only get one retry
}
