// Unit tests for the core client: config, proxy, error types, rate limiter, retry, on_retry.

use lolzteam::client::{ApiClient, RetryConfig, RetryInfo};
use lolzteam::{Error, LolzteamClient, FORUM_BASE_URL, MARKET_BASE_URL};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

// ---------------------------------------------------------------------------
// RetryConfig defaults
// ---------------------------------------------------------------------------

#[test]
fn retry_config_default_values() {
    let cfg = RetryConfig::default();
    assert_eq!(cfg.max_retries, 5);
    assert_eq!(cfg.base_delay_ms, 1000);
    assert_eq!(cfg.max_delay_ms, 60_000);
}

#[test]
fn retry_config_custom_values() {
    let cfg = RetryConfig {
        max_retries: 10,
        base_delay_ms: 2000,
        max_delay_ms: 120_000,
    };
    assert_eq!(cfg.max_retries, 10);
    assert_eq!(cfg.base_delay_ms, 2000);
    assert_eq!(cfg.max_delay_ms, 120_000);
}

// ---------------------------------------------------------------------------
// ApiClient builder
// ---------------------------------------------------------------------------

#[test]
fn api_client_builder_default() {
    let client = ApiClient::builder("https://example.com", "test-token")
        .build()
        .unwrap();
    assert!(format!("{:?}", client).contains("example.com"));
}

#[test]
fn api_client_builder_with_proxy() {
    let client = ApiClient::builder("https://example.com", "test-token")
        .proxy("socks5://127.0.0.1:1080")
        .build()
        .unwrap();
    assert!(format!("{:?}", client).contains("example.com"));
}

#[test]
fn api_client_builder_with_retry_config() {
    let client = ApiClient::builder("https://example.com", "test-token")
        .retry_config(RetryConfig {
            max_retries: 10,
            base_delay_ms: 500,
            max_delay_ms: 30_000,
        })
        .build()
        .unwrap();
    assert!(format!("{:?}", client).contains("example.com"));
}

#[test]
fn api_client_builder_no_retry() {
    let client = ApiClient::builder("https://example.com", "test-token")
        .no_retry()
        .build()
        .unwrap();
    assert!(format!("{:?}", client).contains("example.com"));
}

#[test]
fn api_client_builder_with_rate_limit() {
    let client = ApiClient::builder("https://example.com", "test-token")
        .rate_limit(300)
        .build()
        .unwrap();
    assert!(format!("{:?}", client).contains("has_rate_limiter: true"));
}

#[test]
fn api_client_builder_with_search_rate_limit() {
    let client = ApiClient::builder("https://example.com", "test-token")
        .search_rate_limit(20)
        .build()
        .unwrap();
    assert!(format!("{:?}", client).contains("has_search_rate_limiter: true"));
}

#[test]
fn api_client_builder_with_on_retry() {
    let cb = Arc::new(|_info: RetryInfo| {});
    let client = ApiClient::builder("https://example.com", "test-token")
        .on_retry(cb)
        .build()
        .unwrap();
    assert!(format!("{:?}", client).contains("has_on_retry: true"));
}

#[test]
fn api_client_builder_with_timeout() {
    let _client = ApiClient::builder("https://example.com", "test-token")
        .timeout(Duration::from_secs(60))
        .build()
        .unwrap();
}

#[test]
fn api_client_builder_debug_redacts_token() {
    let builder = ApiClient::builder("https://example.com", "super-secret-token");
    let debug = format!("{:?}", builder);
    assert!(debug.contains("[redacted]"));
    assert!(!debug.contains("super-secret-token"));
}

// ---------------------------------------------------------------------------
// Proxy URL validation
// ---------------------------------------------------------------------------

#[test]
fn proxy_rejects_invalid_url() {
    let result = ApiClient::builder("https://example.com", "t")
        .proxy("not a url")
        .build();
    assert!(result.is_err());
    assert!(matches!(result, Err(Error::Config(_))));
}

#[test]
fn proxy_rejects_unsupported_scheme() {
    let result = ApiClient::builder("https://example.com", "t")
        .proxy("ftp://proxy:8080")
        .build();
    assert!(result.is_err());
    assert!(matches!(result, Err(Error::Config(_))));
}

#[test]
fn proxy_rejects_no_host() {
    let result = ApiClient::builder("https://example.com", "t")
        .proxy("http://")
        .build();
    assert!(result.is_err());
    assert!(matches!(result, Err(Error::Config(_))));
}

#[test]
fn proxy_accepts_valid_http() {
    let result = ApiClient::builder("https://example.com", "t")
        .proxy("http://proxy:8080")
        .build();
    assert!(result.is_ok());
}

#[test]
fn proxy_accepts_valid_https() {
    let result = ApiClient::builder("https://example.com", "t")
        .proxy("https://proxy:8080")
        .build();
    assert!(result.is_ok());
}

#[test]
fn proxy_accepts_valid_socks5() {
    let result = ApiClient::builder("https://example.com", "t")
        .proxy("socks5://127.0.0.1:1080")
        .build();
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// LolzteamClient builder
// ---------------------------------------------------------------------------

#[test]
fn lolzteam_client_new() {
    let client = LolzteamClient::new("test-token").unwrap();
    let _ = client.forum();
    let _ = client.market();
}

#[test]
fn lolzteam_client_builder_default() {
    let client = LolzteamClient::builder("test-token").build().unwrap();
    let _ = client.forum();
    let _ = client.market();
}

#[test]
fn lolzteam_client_builder_with_proxy() {
    let client = LolzteamClient::builder("test-token")
        .proxy("socks5://127.0.0.1:1080")
        .build()
        .unwrap();
    let _ = client.forum();
    let _ = client.market();
}

#[test]
fn lolzteam_client_builder_separate_proxies() {
    let client = LolzteamClient::builder("test-token")
        .forum_proxy("socks5://127.0.0.1:1080")
        .market_proxy("http://proxy:8080")
        .build()
        .unwrap();
    let _ = client.forum();
    let _ = client.market();
}

#[test]
fn lolzteam_client_builder_custom_base_urls() {
    let client = LolzteamClient::builder("test-token")
        .forum_base_url("https://custom-forum.example.com")
        .market_base_url("https://custom-market.example.com")
        .build()
        .unwrap();
    let _ = client.forum();
    let _ = client.market();
}

#[test]
fn lolzteam_client_builder_no_rate_limit() {
    let client = LolzteamClient::builder("test-token")
        .no_rate_limit()
        .build()
        .unwrap();
    let _ = client.forum();
}

#[test]
fn lolzteam_client_builder_custom_rate_limits() {
    let client = LolzteamClient::builder("test-token")
        .forum_rate_limit(600)
        .market_rate_limit(240)
        .market_search_rate_limit(40)
        .build()
        .unwrap();
    let _ = client.forum();
}

#[test]
fn lolzteam_client_builder_custom_retry() {
    let client = LolzteamClient::builder("test-token")
        .max_retries(10)
        .base_delay_ms(500)
        .max_delay_ms(30_000)
        .build()
        .unwrap();
    let _ = client.forum();
}

#[test]
fn lolzteam_client_builder_with_on_retry() {
    let counter = Arc::new(AtomicUsize::new(0));
    let cnt = counter.clone();
    let client = LolzteamClient::builder("test-token")
        .on_retry(Arc::new(move |info: RetryInfo| {
            cnt.fetch_add(1, Ordering::SeqCst);
            println!(
                "retry #{} for {} {} (delay {}ms)",
                info.attempt, info.method, info.path, info.delay_ms
            );
        }))
        .build()
        .unwrap();
    let _ = client.forum();
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

#[test]
fn default_base_urls() {
    assert_eq!(FORUM_BASE_URL, "https://prod-api.lolz.live");
    assert_eq!(MARKET_BASE_URL, "https://prod-api.lzt.market");
}

#[test]
fn default_rate_limits() {
    assert_eq!(lolzteam::FORUM_RATE_LIMIT, 300);
    assert_eq!(lolzteam::MARKET_RATE_LIMIT, 120);
    assert_eq!(lolzteam::MARKET_SEARCH_RATE_LIMIT, 20);
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

#[test]
fn error_auth_status_code() {
    let err = Error::Auth {
        message: "test".into(),
    };
    assert_eq!(err.status_code(), Some(401));
    assert!(err.is_auth());
    assert!(!err.is_retryable());
    assert!(!err.is_rate_limit());
    assert!(!err.is_not_found());
}

#[test]
fn error_forbidden_status_code() {
    let err = Error::Forbidden {
        message: "test".into(),
    };
    assert_eq!(err.status_code(), Some(403));
}

#[test]
fn error_not_found_status_code() {
    let err = Error::NotFound {
        message: "test".into(),
    };
    assert_eq!(err.status_code(), Some(404));
    assert!(err.is_not_found());
    assert!(!err.is_retryable());
}

#[test]
fn error_rate_limited_status_code() {
    let err = Error::RateLimited { attempts: 5 };
    assert_eq!(err.status_code(), Some(429));
    assert!(err.is_rate_limit());
    assert!(err.is_retryable());
}

#[test]
fn error_api_status_code() {
    let err = Error::Api {
        status: 500,
        body: "test".into(),
    };
    assert_eq!(err.status_code(), Some(500));
}

#[test]
fn error_api_429_is_rate_limit() {
    let err = Error::Api {
        status: 429,
        body: "rate limited".into(),
    };
    assert!(err.is_rate_limit());
    assert!(err.is_retryable());
}

#[test]
fn error_api_502_is_retryable() {
    let err = Error::Api {
        status: 502,
        body: "bad gateway".into(),
    };
    assert!(err.is_retryable());
    assert!(!err.is_rate_limit());
}

#[test]
fn error_api_503_is_retryable() {
    let err = Error::Api {
        status: 503,
        body: "service unavailable".into(),
    };
    assert!(err.is_retryable());
}

#[test]
fn error_api_504_is_retryable() {
    let err = Error::Api {
        status: 504,
        body: "gateway timeout".into(),
    };
    assert!(err.is_retryable());
}

#[test]
fn error_api_500_not_retryable() {
    let err = Error::Api {
        status: 500,
        body: "internal".into(),
    };
    assert!(!err.is_retryable());
}

#[test]
fn error_api_401_not_retryable() {
    let err = Error::Auth {
        message: "unauthorized".into(),
    };
    assert!(!err.is_retryable());
}

#[test]
fn error_api_404_not_retryable() {
    let err = Error::NotFound {
        message: "not found".into(),
    };
    assert!(!err.is_retryable());
}

#[test]
fn error_retry_exhausted_wraps_last_error() {
    let inner = Error::Api {
        status: 503,
        body: "service unavailable".into(),
    };
    let err = Error::RetryExhausted {
        attempts: 3,
        last_error: Box::new(inner),
    };
    assert_eq!(err.status_code(), Some(503));
    assert!(format!("{}", err).contains("3 attempts"));
}

#[test]
fn error_config_display() {
    let err = Error::Config("bad proxy".into());
    assert!(format!("{}", err).contains("bad proxy"));
    assert!(err.status_code().is_none());
}

#[test]
fn error_display_formats_correctly() {
    let errors = vec![
        (
            Error::Auth {
                message: "bad token".into(),
            },
            "auth error: bad token",
        ),
        (
            Error::Forbidden {
                message: "denied".into(),
            },
            "forbidden: denied",
        ),
        (
            Error::NotFound {
                message: "gone".into(),
            },
            "not found: gone",
        ),
        (Error::RateLimited { attempts: 5 }, "rate limited after 5"),
        (
            Error::Api {
                status: 500,
                body: "oops".into(),
            },
            "API error 500: oops",
        ),
        (Error::Config("bad".into()), "configuration error: bad"),
    ];
    for (err, expected_substr) in errors {
        let msg = format!("{}", err);
        assert!(
            msg.contains(expected_substr),
            "expected '{}' to contain '{}'",
            msg,
            expected_substr
        );
    }
}

// ---------------------------------------------------------------------------
// Both clients coexist
// ---------------------------------------------------------------------------

#[test]
fn both_clients_can_coexist() {
    let client = LolzteamClient::new("test-token").unwrap();
    let _forum = client.forum();
    let _market = client.market();
    // They should have different base URLs
}
