<p align="center">
  <a href="../README.md">Русский</a> · <a href="DOCS_EN.md">Documentation</a>
</p>

# lolzteam

Rust client for [LOLZTEAM API](https://github.com/AS7RIDENIED/LOLZTEAM) (forum + market).
Methods and types are auto-generated from OpenAPI schemas.

## Features

- 266 endpoints — Forum (151) + Market (115)
- Typed response models
- Proxy — HTTP / HTTPS / SOCKS5 (separate for forum and market)
- Auto-retry on 429 / 502 / 503 with exponential backoff
- Code generation from OpenAPI JSON
- CI/CD — GitHub Actions

## Installation

```toml
[dependencies]
lolzteam = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LolzteamClient::new("YOUR_TOKEN");

    let user = client.forum().users_get(1, None).await?;
    println!("{:?}", user);

    let threads = client.forum().threads_list(Default::default()).await?;
    println!("{:?}", threads);

    let items = client.market().category_steam(Default::default()).await?;
    println!("{:?}", items);

    Ok(())
}
```

## Proxy

```rust
use lolzteam::LolzteamClient;

let client = LolzteamClient::builder("YOUR_TOKEN")
    .proxy("socks5://127.0.0.1:1080")
    // or separate proxies for forum and market:
    // .forum_proxy("socks5://127.0.0.1:1080")
    // .market_proxy("http://user:pass@proxy.example.com:8080")
    .max_retries(3)
    .build()?;
```

## Error Handling

```rust
use lolzteam::Error;

match client.forum().users_get(1, None).await {
    Ok(resp) => println!("{:?}", resp),
    Err(Error::Auth { message }) => eprintln!("401: {}", message),
    Err(Error::Forbidden { message }) => eprintln!("403: {}", message),
    Err(Error::NotFound { message }) => eprintln!("404: {}", message),
    Err(Error::RateLimited { attempts }) => eprintln!("429: {} attempts", attempts),
    Err(e) => eprintln!("other: {}", e),
}
```

## Retry

Automatic retry with exponential backoff:
- **429** — respects `Retry-After` header
- **502 / 503** — transient server errors

Default: up to 5 attempts, starting at 2s, max 60s.

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

## Code Generation

Code is already generated, but you can regenerate from fresh schemas:

```bash
curl -sL https://raw.githubusercontent.com/AS7RIDENIED/LOLZTEAM/main/Official%20Documentation/forum.json -o schemas/forum.json
curl -sL https://raw.githubusercontent.com/AS7RIDENIED/LOLZTEAM/main/Official%20Documentation/market.json -o schemas/market.json

make generate
```

The codegen (`codegen/`) reads OpenAPI 3.1 JSON and generates:
- `src/models.rs` — data models and response wrappers
- `src/forum/{types,methods}.rs`
- `src/market/{types,methods}.rs`

## CI/CD

- `ci.yml` — fmt + clippy + build on every push/PR, codegen freshness check
- `release.yml` — publish to crates.io on GitHub Release

Publishing requires a `CARGO_REGISTRY_TOKEN` secret in repository settings.

## License

MIT
