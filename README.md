<p align="center">
  <a href="docs/README_EN.md">English</a> · <a href="docs/DOCS_RU.md">Документация</a>
</p>

# lolzteam

Rust клиент для [LOLZTEAM API](https://github.com/AS7RIDENIED/LOLZTEAM) (форум + маркет).
Методы и типы сгенерированы из OpenAPI-схем.

## Возможности

- 266 эндпоинтов — Forum (151) + Market (115)
- Типизированные модели ответов
- Типизированный multipart upload для forum-аватаров/обложек
- Прокси — HTTP / HTTPS / SOCKS5 (раздельные для форума и маркета)
- Авто-ретрай на 429 / 502 / 503 / 504 и транзиентные сетевые ошибки
- Кодогенерация из OpenAPI JSON
- CI/CD — GitHub Actions

## Соответствие ТЗ

- Генерация методов и схем ответов из OpenAPI (`codegen/` + `make generate`)
- Forum + Market в одном крейте
- Раздельные прокси для forum/market и общий `.proxy(...)`
- Автоматический ретрай 429 / 502 / 503 (дополнительно 504 и connect/timeout)
- MIT лицензия
- GitHub Actions: fmt, clippy, test, build, `cargo publish --dry-run`, проверка актуальности кодогена

## Установка

```toml
[dependencies]
lolzteam = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Использование

```rust
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LolzteamClient::new("YOUR_TOKEN")?;

    let user = client.forum().users_get(1, None).await?;
    println!("{:?}", user);

    let threads = client.forum().threads_list(Default::default()).await?;
    println!("{:?}", threads);

    let items = client.market().category_steam(Default::default()).await?;
    println!("{:?}", items);

    Ok(())
}
```

## Прокси

```rust
use lolzteam::LolzteamClient;

let client = LolzteamClient::builder("YOUR_TOKEN")
    .proxy("socks5://127.0.0.1:1080")
    // или разные прокси для форума и маркета:
    // .forum_proxy("socks5://127.0.0.1:1080")
    // .market_proxy("http://user:pass@proxy.example.com:8080")
    .max_retries(3)
    .build()?;
```

## Обработка ошибок

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

## Ретрай

Автоматический ретрай с экспоненциальным бэкоффом:
- **429** — учитывает `Retry-After`
- **502 / 503 / 504** — транзиентные ошибки сервера
- **timeout / connect** — сетевые ошибки транспорта

По умолчанию до 5 попыток, начиная с 2с, максимум 60с.

## Multipart upload

```rust
use lolzteam::client::MultipartFile;
use lolzteam::forum::types::ForumUsersAvatarUploadParams;

let avatar = MultipartFile::new(std::fs::read("avatar.png")?)
    .with_filename("avatar.png");

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


## Request bodies

- `RequestBody::Json(...)` — JSON payloads
- `RequestBody::Form(...)` — x-www-form-urlencoded payloads
- `RequestBody::Multipart(...)` — multipart uploads with `MultipartForm`, `MultipartFile`

Example multipart upload:

```rust
use lolzteam::{MultipartFile, MultipartForm, RequestBody};
use lolzteam::forum::types::ForumUsersAvatarUploadParams;

let mut form = MultipartForm::new();
form.file("avatar", MultipartFile::new(std::fs::read("avatar.png")?)
    .with_filename("avatar.png")
    .with_mime_type("image/png"));
form.text("crop", "256");

let params = ForumUsersAvatarUploadParams {
    avatar: form,
    crop: Some(256),
    x: Some(0),
    y: Some(0),
};

client.forum().users_avatar_upload(1, params).await?;
```

## Endpoint parameters

Endpoints with more than three optional parameters accept a `*Params` struct.

```rust
use lolzteam::market::types::MarketCategorySteamParams;

let params = MarketCategorySteamParams {
    pmin: Some(10),
    pmax: Some(100),
    ..Default::default()
};
let results = client.market().category_steam(params).await?;
```

## Параметры эндпоинтов

Эндпоинты с >3 опциональными параметрами принимают `*Params` структуру:

```rust
use lolzteam::market::types::MarketCategorySteamParams;

let params = MarketCategorySteamParams {
    pmin: Some(10),
    pmax: Some(100),
    ..Default::default()
};
let results = client.market().category_steam(params).await?;
```

## Кодогенерация

Код уже сгенерирован, но можно перегенерировать из свежих схем:

```bash
curl -sL https://raw.githubusercontent.com/AS7RIDENIED/LOLZTEAM/main/Official%20Documentation/forum.json -o schemas/forum.json
curl -sL https://raw.githubusercontent.com/AS7RIDENIED/LOLZTEAM/main/Official%20Documentation/market.json -o schemas/market.json

make generate
```

Кодогенератор (`codegen/`) читает OpenAPI 3.1 JSON и генерирует:
- `src/models.rs` — модели данных и response-обёртки
- `src/forum/{types,methods}.rs`
- `src/market/{types,methods}.rs`

## CI/CD

- `ci.yml` — fmt + clippy + build на каждый push/PR, проверка актуальности кодогена
- `release.yml` — публикация на crates.io при создании GitHub Release

Для публикации нужен секрет `CARGO_REGISTRY_TOKEN` в настройках репозитория.

## Лицензия

MIT


