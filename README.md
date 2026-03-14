# lolzteam

Rust клиент для [LOLZTEAM API](https://github.com/AS7RIDENIED/LOLZTEAM) (форум + маркет).
Методы и типы сгенерированы из OpenAPI-схем.

## Возможности

- 266 эндпоинтов — Forum (151) + Market (115)
- Прокси — HTTP / HTTPS / SOCKS5
- Авто-ретрай на 429 / 502 / 503 с экспоненциальным бэкоффом
- Типизированные модели ответов (serde)
- Кодогенерация из OpenAPI JSON

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

## Ретрай

Автоматический ретрай с экспоненциальным бэкоффом:
- **429** — учитывает `Retry-After`
- **502 / 503** — транзиентные ошибки сервера

По умолчанию до 5 попыток, начиная с 2с, максимум 60с.

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
- `src/models.rs`
- `src/forum/{types,methods}.rs`
- `src/market/{types,methods}.rs`

## CI/CD

- `ci.yml` — fmt + clippy + build + test на каждый push/PR, проверка актуальности кодогена
- `release.yml` — публикация на crates.io по тегу `v*`

```bash
git tag v0.1.0
git push origin v0.1.0
```

Нужен секрет `CARGO_REGISTRY_TOKEN` в настройках репозитория.

## Лицензия

MIT
