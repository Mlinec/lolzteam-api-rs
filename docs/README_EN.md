<p align="center">
  <a href="../README.md">Русский</a> · <a href="DOCS_EN.md">Documentation</a>
</p>

# lolzteam

Full-featured Rust client for [LOLZTEAM API](https://github.com/AS7RIDENIED/LOLZTEAM) (Forum + Market).
All methods and types are auto-generated from OpenAPI schemas.

## Key Features

| Feature | Details |
|---|---|
| 🔗 **266 endpoints** | Forum (151) + Market (115) — full API coverage |
| 🧩 **211 types** | 23 enums + 188 structs, auto-generated with `Serialize` / `Deserialize` |
| 📤 **Multipart upload** | Typed API for avatars, covers, and file uploads |
| 🌐 **Proxy** | HTTP / HTTPS / SOCKS5 with URL validation, separate for forum and market |
| ♻️ **Auto-retry** | 429 / 502 / 503 / 504 + network errors, exponential backoff with jitter |
| 🔔 **on_retry callback** | Real-time monitoring of every retry attempt |
| 🪣 **Token-bucket Rate Limiter** | Proactive request throttling (Forum: 300/min, Market: 120/min) |
| 🔍 **Search Rate Limit** | Separate limit for search endpoints (Market: 20/min) |
| 🔒 **Token safety** | Automatic token redaction in `Debug` output |
| ⚡ **Code generation** | From OpenAPI 3.1 JSON (`codegen/`) |
| 🧪 **168 tests** | Unit + mock server + live API + integration |
| 🚀 **CI/CD** | GitHub Actions: fmt, clippy, test, build, dry-run publish, codegen check |

## Quick Start

### Installation

```toml
[dependencies]
lolzteam = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Minimal Example

```rust
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LolzteamClient::new("YOUR_TOKEN")?;

    // Forum API
    let user = client.forum().users_get(1, None).await?;
    println!("{:?}", user);

    // Market API
    let items = client.market().category_steam(Default::default()).await?;
    println!("{:?}", items);

    Ok(())
}
```

## Configuration

### Full Builder

```rust
use lolzteam::{LolzteamClient, RetryInfo};
use std::sync::Arc;

let client = LolzteamClient::builder("YOUR_TOKEN")
    // Proxy
    .forum_proxy("socks5://127.0.0.1:1080")
    .market_proxy("http://user:pass@proxy.example.com:8080")
    // Retry
    .max_retries(5)
    .base_delay_ms(1000)
    .max_delay_ms(60_000)
    // Rate limiting
    .forum_rate_limit(300)       // 300 req/min
    .market_rate_limit(120)      // 120 req/min
    .market_search_rate_limit(20) // 20 searches/min
    // Retry callback
    .on_retry(Arc::new(|info: RetryInfo| {
        eprintln!(
            "⚠️  Retry #{} {} {} (status: {:?}, delay: {}ms)",
            info.attempt, info.method, info.path, info.status, info.delay_ms
        );
    }))
    .build()?;
```

### Builder Options

| Method | Description | Default |
|---|---|---|
| `.proxy(url)` | Shared proxy for both APIs | — |
| `.forum_proxy(url)` | Forum API proxy | — |
| `.market_proxy(url)` | Market API proxy | — |
| `.max_retries(n)` | Maximum retry attempts | `5` |
| `.base_delay_ms(ms)` | Initial backoff delay | `1000` |
| `.max_delay_ms(ms)` | Maximum backoff delay | `60000` |
| `.timeout(dur)` | Request timeout | `30s` |
| `.forum_rate_limit(rpm)` | Forum rate limit (req/min) | `300` |
| `.market_rate_limit(rpm)` | Market rate limit (req/min) | `120` |
| `.market_search_rate_limit(rpm)` | Market search rate limit (req/min) | `20` |
| `.no_rate_limit()` | Disable all rate limiters | — |
| `.on_retry(cb)` | Callback on each retry | — |
| `.forum_base_url(url)` | Override Forum URL | `https://prod-api.lolz.live` |
| `.market_base_url(url)` | Override Market URL | `https://prod-api.lzt.market` |

## Error Handling

```rust
use lolzteam::Error;

match client.forum().users_get(1, None).await {
    Ok(resp) => println!("{:?}", resp),
    Err(Error::Auth { message }) => eprintln!("401 Unauthorized: {}", message),
    Err(Error::Forbidden { message }) => eprintln!("403 Forbidden: {}", message),
    Err(Error::NotFound { message }) => eprintln!("404 Not Found: {}", message),
    Err(Error::RateLimited { attempts }) => eprintln!("429 Rate Limited ({} attempts)", attempts),
    Err(Error::RetryExhausted { attempts, last_error }) => {
        eprintln!("All {} attempts exhausted. Last error: {}", attempts, last_error);
    }
    Err(Error::Config(msg)) => eprintln!("Configuration error: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}

// Helper methods
let err = Error::Api { status: 429, body: "rate limited".into() };
assert!(err.is_retryable());     // true for 429/502/503/504
assert!(err.is_rate_limit());    // true for 429
assert_eq!(err.status_code(), Some(429));
```

### Error Types

| Variant | Status | `is_retryable()` | Description |
|---|---|---|---|
| `Http` | — | ✅ (timeout/connect) | reqwest error |
| `Json` | — | ❌ | JSON parsing error |
| `Api` | `status` | ✅ (429/502/503/504) | Generic API error |
| `Auth` | 401 | ❌ | Invalid token |
| `Forbidden` | 403 | ❌ | Access denied |
| `NotFound` | 404 | ❌ | Resource not found |
| `RateLimited` | 429 | ✅ | Rate limit exceeded |
| `RetryExhausted` | * | — | All retries failed (wraps `last_error`) |
| `Config` | — | ❌ | Configuration error |

## Rate Limiting

Token-bucket algorithm with continuous refill:

```rust
// Client automatically throttles requests
let client = LolzteamClient::new("YOUR_TOKEN")?;
// Forum: 300 req/min (5/sec with burst up to 300)
// Market: 120 req/min (2/sec with burst up to 120)
// Market search: 20 req/min (separate limiter)

// Disable rate limiting:
let client = LolzteamClient::builder("YOUR_TOKEN")
    .no_rate_limit()
    .build()?;
```

Search endpoints automatically pass through **two** limiters: the standard limiter + the search limiter.

## Retry

Exponential backoff with jitter (built-in PRNG, no external dependencies):
- **429** — respects `Retry-After` header (seconds and HTTP-date)
- **502 / 503 / 504** — transient server errors
- **timeout / connect** — transport network errors

Delay formula: `min(base_delay × 2^attempt + random_jitter, max_delay)`

## Proxy

```rust
use lolzteam::LolzteamClient;

// Shared proxy for both APIs
let client = LolzteamClient::builder("YOUR_TOKEN")
    .proxy("socks5://127.0.0.1:1080")
    .build()?;

// Separate proxies
let client = LolzteamClient::builder("YOUR_TOKEN")
    .forum_proxy("socks5://127.0.0.1:1080")
    .market_proxy("http://user:pass@proxy.example.com:8080")
    .build()?;
```

Supported schemes: `http://`, `https://`, `socks5://`. URLs are validated at client creation time.

## Multipart Upload

```rust
use lolzteam::client::MultipartFile;
use lolzteam::forum::types::ForumUsersAvatarUploadParams;

let avatar = MultipartFile::new(std::fs::read("avatar.png")?)
    .with_filename("avatar.png")
    .with_mime_type("image/png");

client.forum().users_avatar_upload(
    1,
    ForumUsersAvatarUploadParams {
        avatar,
        crop: Some(256),
        x: Some(0),
        y: Some(0),
    },
).await?;
```

## Endpoint Parameters

Endpoints with >3 optional parameters accept a `*Params` struct:

```rust
use lolzteam::market::types::MarketCategorySteamParams;

let params = MarketCategorySteamParams {
    pmin: Some(10),
    pmax: Some(100),
    ..Default::default()
};
let results = client.market().category_steam(params).await?;
```

## Project Structure

```
lolzteam-api-rs/
├── src/
│   ├── lib.rs          # Public API: LolzteamClient, LolzteamClientBuilder
│   ├── client.rs       # HTTP client, rate limiter, retry, proxy
│   ├── error.rs        # Typed errors with helper methods
│   ├── models.rs       # 211 types: 23 enums + 188 structs (generated)
│   ├── forum/          # Forum API: 151 endpoints (generated)
│   │   ├── methods.rs
│   │   ├── types.rs
│   │   └── mod.rs
│   └── market/         # Market API: 115 endpoints (generated)
│       ├── methods.rs
│       ├── types.rs
│       └── mod.rs
├── codegen/            # OpenAPI → Rust code generator
│   └── src/main.rs
├── schemas/            # OpenAPI 3.1 JSON schemas
│   ├── forum.json
│   └── market.json
├── tests/              # 168 tests
│   ├── client_tests.rs                # 44 tests: builder, proxy, error types
│   ├── rate_limiter_tests.rs          # 9 tests: rate limiter, retry, on_retry
│   ├── mock_server_tests.rs           # 21 tests: TCP mock server (429, 502, 503, 504, Retry-After, backoff)
│   ├── live_api_tests.rs             # 50 tests: real API (Forum + Market)
│   ├── integration_test.rs            # 43 tests: endpoint signatures
│   └── retry_multipart_regression.rs  # 1 test: retry + multipart
├── examples/           # Usage examples
│   ├── general/        # Basic usage, proxy, error handling
│   ├── forum/          # Forum API examples
│   └── market/         # Market API examples
└── .github/workflows/  # CI/CD
    ├── ci.yml          # fmt + clippy + test + build + codegen check
    └── release.yml     # Publish to crates.io
```

## Code Generation

Code is already generated. Regenerate from fresh schemas:

```bash
curl -sL https://raw.githubusercontent.com/AS7RIDENIED/LOLZTEAM/main/Official%20Documentation/forum.json -o schemas/forum.json
curl -sL https://raw.githubusercontent.com/AS7RIDENIED/LOLZTEAM/main/Official%20Documentation/market.json -o schemas/market.json

make generate   # cargo run -p lolzteam-codegen -- schemas/forum.json schemas/market.json src
```

## CI/CD

- **ci.yml** — on every push/PR: `cargo fmt`, `cargo clippy -D warnings`, `cargo test`, `cargo build`, `cargo publish --dry-run`, codegen freshness check
- **release.yml** — publish to crates.io on GitHub Release creation

Publishing requires a `CARGO_REGISTRY_TOKEN` secret in repository settings.

## License

MIT


