<p align="center">
  <a href="docs/README_EN.md">English</a> · <a href="docs/DOCS_RU.md">Документация</a>
</p>

# lolzteam

Полноценный Rust-клиент для [LOLZTEAM API](https://github.com/AS7RIDENIED/LOLZTEAM) (Forum + Market).
Все методы и типы автоматически сгенерированы из OpenAPI-схем.

## Ключевые возможности

| Возможность | Подробности |
|---|---|
| 🔗 **266 эндпоинтов** | Forum (151) + Market (115) — полное покрытие API |
| 🧩 **211 типов** | 23 enum + 188 struct, автогенерация с `Serialize` / `Deserialize` |
| 📤 **Multipart upload** | Загрузка аватаров, обложек и файлов через типизированный API |
| 🌐 **Прокси** | HTTP / HTTPS / SOCKS5 с валидацией URL, раздельные для форума и маркета |
| ♻️ **Авто-ретрай** | 429 / 502 / 503 / 504 + сетевые ошибки, экспоненциальный бэкофф с jitter |
| 🔔 **on_retry колбэк** | Мониторинг каждой повторной попытки в реальном времени |
| 🪣 **Token-bucket Rate Limiter** | Проактивное ограничение запросов (Forum: 300/мин, Market: 120/мин) |
| 🔍 **Search Rate Limit** | Отдельный лимит для поисковых эндпоинтов (Market: 20/мин) |
| 🔒 **Безопасность токена** | Автоматическая редакция токена в `Debug`-выводе |
| ⚡ **Кодогенерация** | Из OpenAPI 3.1 JSON (`codegen/`) |
| 🧪 **168 тестов** | Unit + mock-сервер + live API + integration |
| 🚀 **CI/CD** | GitHub Actions: fmt, clippy, test, build, dry-run publish, проверка кодогена |

## Быстрый старт

### Установка

```toml
[dependencies]
lolzteam = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Минимальный пример

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

## Конфигурация

### Полный конфигуратор

```rust
use lolzteam::{LolzteamClient, RetryInfo};
use std::sync::Arc;

let client = LolzteamClient::builder("YOUR_TOKEN")
    // Прокси
    .forum_proxy("socks5://127.0.0.1:1080")
    .market_proxy("http://user:pass@proxy.example.com:8080")
    // Ретрай
    .max_retries(5)
    .base_delay_ms(1000)
    .max_delay_ms(60_000)
    // Rate limiting
    .forum_rate_limit(300)       // 300 запросов/мин
    .market_rate_limit(120)      // 120 запросов/мин
    .market_search_rate_limit(20) // 20 поисков/мин
    // Колбэк при ретрае
    .on_retry(Arc::new(|info: RetryInfo| {
        eprintln!(
            "⚠️  Ретрай #{} {} {} (статус: {:?}, задержка: {}ms)",
            info.attempt, info.method, info.path, info.status, info.delay_ms
        );
    }))
    .build()?;
```

### Параметры конфигуратора

| Метод | Описание | По умолчанию |
|---|---|---|
| `.proxy(url)` | Общий прокси для обоих API | — |
| `.forum_proxy(url)` | Прокси для Forum API | — |
| `.market_proxy(url)` | Прокси для Market API | — |
| `.max_retries(n)` | Макс. число повторных попыток | `5` |
| `.base_delay_ms(ms)` | Начальная задержка бэкоффа | `1000` |
| `.max_delay_ms(ms)` | Макс. задержка бэкоффа | `60000` |
| `.timeout(dur)` | Таймаут запроса | `30s` |
| `.forum_rate_limit(rpm)` | Лимит Forum (запросов/мин) | `300` |
| `.market_rate_limit(rpm)` | Лимит Market (запросов/мин) | `120` |
| `.market_search_rate_limit(rpm)` | Лимит поиска Market (запросов/мин) | `20` |
| `.no_rate_limit()` | Отключить все rate limiter-ы | — |
| `.on_retry(cb)` | Колбэк при каждом ретрае | — |
| `.forum_base_url(url)` | Переопределение URL форума | `https://prod-api.lolz.live` |
| `.market_base_url(url)` | Переопределение URL маркета | `https://prod-api.lzt.market` |

## Обработка ошибок

```rust
use lolzteam::Error;

match client.forum().users_get(1, None).await {
    Ok(resp) => println!("{:?}", resp),
    Err(Error::Auth { message }) => eprintln!("401 Unauthorized: {}", message),
    Err(Error::Forbidden { message }) => eprintln!("403 Forbidden: {}", message),
    Err(Error::NotFound { message }) => eprintln!("404 Not Found: {}", message),
    Err(Error::RateLimited { attempts }) => eprintln!("429 Rate Limited ({} попыток)", attempts),
    Err(Error::RetryExhausted { attempts, last_error }) => {
        eprintln!("Все {} попытки исчерпаны. Последняя ошибка: {}", attempts, last_error);
    }
    Err(Error::Config(msg)) => eprintln!("Ошибка конфигурации: {}", msg),
    Err(e) => eprintln!("Ошибка: {}", e),
}

// Вспомогательные методы
let err = Error::Api { status: 429, body: "rate limited".into() };
assert!(err.is_retryable());     // true для 429/502/503/504
assert!(err.is_rate_limit());    // true для 429
assert_eq!(err.status_code(), Some(429));
```

### Типы ошибок

| Вариант | Статус | `is_retryable()` | Описание |
|---|---|---|---|
| `Http` | — | ✅ (timeout/connect) | Ошибка reqwest |
| `Json` | — | ❌ | Ошибка парсинга JSON |
| `Api` | `status` | ✅ (429/502/503/504) | Общая ошибка API |
| `Auth` | 401 | ❌ | Недействительный токен |
| `Forbidden` | 403 | ❌ | Нет доступа |
| `NotFound` | 404 | ❌ | Ресурс не найден |
| `RateLimited` | 429 | ✅ | Лимит запросов превышен |
| `RetryExhausted` | * | — | Все попытки исчерпаны (содержит `last_error`) |
| `Config` | — | ❌ | Ошибка конфигурации |

## Rate Limiting

Token-bucket алгоритм с непрерывным восполнением:

```rust
// Клиент автоматически ограничивает запросы
let client = LolzteamClient::new("YOUR_TOKEN")?;
// Forum: 300 запросов/мин (5/сек с burst до 300)
// Market: 120 запросов/мин (2/сек с burst до 120)
// Market search: 20 запросов/мин (отдельный лимитер)

// Отключить rate limiting:
let client = LolzteamClient::builder("YOUR_TOKEN")
    .no_rate_limit()
    .build()?;
```

Поисковые эндпоинты автоматически проходят через **два** лимитера: общий + поисковый.

## Ретрай

Экспоненциальный бэкофф с jitter (встроенный PRNG, без внешних зависимостей):
- **429** — учитывает заголовок `Retry-After` (секунды и HTTP-date)
- **502 / 503 / 504** — транзиентные ошибки сервера
- **timeout / connect** — сетевые ошибки транспорта

Формула задержки: `min(base_delay × 2^attempt + random_jitter, max_delay)`

## Прокси

```rust
use lolzteam::LolzteamClient;

// Общий прокси для обоих API
let client = LolzteamClient::builder("YOUR_TOKEN")
    .proxy("socks5://127.0.0.1:1080")
    .build()?;

// Раздельные прокси
let client = LolzteamClient::builder("YOUR_TOKEN")
    .forum_proxy("socks5://127.0.0.1:1080")
    .market_proxy("http://user:pass@proxy.example.com:8080")
    .build()?;
```

Поддерживаются схемы `http://`, `https://`, `socks5://`. URL валидируются при создании клиента.

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

## Структура проекта

```
lolzteam-api-rs/
├── src/
│   ├── lib.rs          # Публичный API: LolzteamClient, LolzteamClientBuilder
│   ├── client.rs       # HTTP-клиент, rate limiter, retry, прокси
│   ├── error.rs        # Типизированные ошибки с helper-методами
│   ├── models.rs       # 211 типов: 23 enum + 188 struct (сгенерировано)
│   ├── forum/          # Forum API: 151 эндпоинт (сгенерировано)
│   │   ├── methods.rs
│   │   ├── types.rs
│   │   └── mod.rs
│   └── market/         # Market API: 115 эндпоинтов (сгенерировано)
│       ├── methods.rs
│       ├── types.rs
│       └── mod.rs
├── codegen/            # OpenAPI → Rust кодогенератор
│   └── src/main.rs
├── schemas/            # OpenAPI 3.1 JSON схемы
│   ├── forum.json
│   └── market.json
├── tests/              # 168 тестов
│   ├── client_tests.rs                # 44 теста: builder, proxy, error types
│   ├── rate_limiter_tests.rs          # 9 тестов: rate limiter, retry, on_retry
│   ├── mock_server_tests.rs           # 21 тест: TCP mock-сервер (429, 502, 503, 504, Retry-After, backoff)
│   ├── live_api_tests.rs             # 50 тестов: реальное API (Forum + Market)
│   ├── integration_test.rs            # 43 теста: сигнатуры эндпоинтов
│   └── retry_multipart_regression.rs  # 1 тест: retry + multipart
├── examples/           # Примеры использования
│   ├── general/        # Базовые примеры, прокси, ошибки
│   ├── forum/          # Примеры Forum API
│   └── market/         # Примеры Market API
└── .github/workflows/  # CI/CD
    ├── ci.yml          # fmt + clippy + test + build + codegen check
    └── release.yml     # Публикация на crates.io
```

## Все эндпоинты

### Forum API (151 эндпоинт)

<details>
<summary>📁 Assets (1)</summary>

| Метод | Описание |
|---|---|
| `assets_css()` | Получить CSS |

</details>

<details>
<summary>🔑 Authentication (1)</summary>

| Метод | Описание |
|---|---|
| `o_auth_token(...)` | Получить Access Token |

</details>

<details>
<summary>📦 Batch (1)</summary>

| Метод | Описание |
|---|---|
| `batch_execute(...)` | Пакетное выполнение запросов |

</details>

<details>
<summary>📂 Categories (2)</summary>

| Метод | Описание |
|---|---|
| `categories_get(id)` | Получить категорию |
| `categories_list(...)` | Получить все категории |

</details>

<details>
<summary>💬 Chatbox (12)</summary>

| Метод | Описание |
|---|---|
| `chatbox_delete_ignore(...)` | Разигнорировать пользователя |
| `chatbox_delete_message(...)` | Удалить сообщение |
| `chatbox_edit_message(...)` | Редактировать сообщение |
| `chatbox_get_ignore(...)` | Список игнорируемых |
| `chatbox_get_leaderboard(...)` | Лидерборд чата |
| `chatbox_get_messages(...)` | Получить сообщения чата |
| `chatbox_index(...)` | Получить чаты |
| `chatbox_online(...)` | Онлайн в чате |
| `chatbox_post_ignore(...)` | Игнорировать пользователя |
| `chatbox_post_message(...)` | Создать сообщение |
| `chatbox_report(...)` | Пожаловаться на сообщение |
| `chatbox_report_reasons(...)` | Причины жалоб |

</details>

<details>
<summary>🏷️ Tags (4)</summary>

| Метод | Описание |
|---|---|
| `tags_find(tag)` | Поиск контента по тегу |
| `tags_get(...)` | Получить контент по тегу |
| `tags_list(...)` | Список тегов |
| `tags_popular()` | Популярные теги |

</details>

<details>
<summary>✉️ Conversations (21)</summary>

| Метод | Описание |
|---|---|
| `conversations_alerts_disable(...)` | Отключить уведомления |
| `conversations_alerts_enable(...)` | Включить уведомления |
| `conversations_create(...)` | Создать диалог |
| `conversations_delete(...)` | Покинуть диалог |
| `conversations_get(id)` | Получить диалог |
| `conversations_invite(...)` | Пригласить пользователей |
| `conversations_kick(...)` | Кикнуть пользователя |
| `conversations_list(...)` | Список диалогов |
| `conversations_messages_create(...)` | Создать сообщение |
| `conversations_messages_delete(...)` | Удалить сообщение |
| `conversations_messages_edit(...)` | Редактировать сообщение |
| `conversations_messages_get(...)` | Получить сообщение |
| `conversations_messages_list(...)` | Список сообщений |
| `conversations_messages_stick(...)` | Закрепить сообщение |
| `conversations_messages_unstick(...)` | Открепить сообщение |
| `conversations_read(...)` | Прочитать диалог |
| `conversations_read_all()` | Прочитать все диалоги |
| `conversations_save(...)` | Сохранить в Saved |
| `conversations_search(...)` | Поиск сообщений |
| `conversations_star(...)` | Пометить избранным |
| `conversations_start(...)` | Начать диалог |

</details>

<details>
<summary>📝 Forms (2)</summary>

| Метод | Описание |
|---|---|
| `forms_create(...)` | Создать форму |
| `forms_list(...)` | Список форм |

</details>

<details>
<summary>🏛️ Forums (9)</summary>

| Метод | Описание |
|---|---|
| `forums_edit_feed_options(...)` | Редактировать ленту |
| `forums_follow(...)` | Подписаться на форум |
| `forums_followed(...)` | Подписанные форумы |
| `forums_followers(...)` | Подписчики форума |
| `forums_get(id)` | Получить форум |
| `forums_get_feed_options()` | Параметры ленты |
| `forums_grouped()` | Дерево форумов |
| `forums_list(...)` | Список форумов |
| `forums_unfollow(...)` | Отписаться от форума |

</details>

<details>
<summary>🔗 Links & Navigation (3)</summary>

| Метод | Описание |
|---|---|
| `links_get(...)` | Получить ссылку форума |
| `links_list(...)` | Список ссылок |
| `navigation_list(...)` | Навигация |

</details>

<details>
<summary>🔔 Notifications (3)</summary>

| Метод | Описание |
|---|---|
| `notifications_get(id)` | Получить уведомление |
| `notifications_list(...)` | Список уведомлений |
| `notifications_read(id)` | Пометить прочитанным |

</details>

<details>
<summary>📄 Pages (2)</summary>

| Метод | Описание |
|---|---|
| `pages_get(...)` | Получить страницу |
| `pages_list(...)` | Список страниц |

</details>

<details>
<summary>💬 Posts (15)</summary>

| Метод | Описание |
|---|---|
| `posts_comments_create(...)` | Создать комментарий |
| `posts_comments_delete(...)` | Удалить комментарий |
| `posts_comments_edit(...)` | Редактировать комментарий |
| `posts_comments_get(...)` | Получить комментарии |
| `posts_comments_report(...)` | Пожаловаться на комментарий |
| `posts_create(...)` | Создать пост |
| `posts_delete(id)` | Удалить пост |
| `posts_edit(...)` | Редактировать пост |
| `posts_get(id)` | Получить пост |
| `posts_like(id)` | Лайкнуть пост |
| `posts_likes(id)` | Лайки поста |
| `posts_list(...)` | Список постов |
| `posts_report(...)` | Пожаловаться на пост |
| `posts_report_reasons(...)` | Причины жалоб |
| `posts_unlike(id)` | Убрать лайк |

</details>

<details>
<summary>👤 Profile Posts (16)</summary>

| Метод | Описание |
|---|---|
| `profile_posts_comments_create(...)` | Комментарий к записи |
| `profile_posts_comments_delete(...)` | Удалить комментарий |
| `profile_posts_comments_edit(...)` | Редактировать комментарий |
| `profile_posts_comments_get(...)` | Получить комментарий |
| `profile_posts_comments_list(...)` | Список комментариев |
| `profile_posts_comments_report(...)` | Пожаловаться |
| `profile_posts_create(...)` | Создать запись |
| `profile_posts_delete(id)` | Удалить запись |
| `profile_posts_edit(...)` | Редактировать запись |
| `profile_posts_get(id)` | Получить запись |
| `profile_posts_like(id)` | Лайкнуть |
| `profile_posts_likes(id)` | Лайки записи |
| `profile_posts_list(...)` | Список записей |
| `profile_posts_report(...)` | Пожаловаться |
| `profile_posts_stick(id)` | Закрепить запись |
| `profile_posts_unstick(id)` | Открепить запись |

</details>

<details>
<summary>🔍 Search (7)</summary>

| Метод | Описание |
|---|---|
| `search_all(...)` | Поиск |
| `search_posts(...)` | Поиск постов |
| `search_profile_posts(...)` | Поиск записей профиля |
| `search_results(...)` | Результаты поиска |
| `search_tagged(...)` | Поиск по тегам |
| `search_threads(...)` | Поиск тем |
| `search_users(...)` | Поиск пользователей |

</details>

<details>
<summary>📋 Threads (22)</summary>

| Метод | Описание |
|---|---|
| `threads_bump(id)` | Бампнуть тему |
| `threads_claim(...)` | Создать жалобу |
| `threads_create(...)` | Создать тему |
| `threads_create_contest(...)` | Создать конкурс |
| `threads_delete(id)` | Удалить тему |
| `threads_edit(...)` | Редактировать тему |
| `threads_finish(...)` | Завершить конкурс |
| `threads_follow(id)` | Подписаться |
| `threads_followed(...)` | Подписанные темы |
| `threads_followers(id)` | Подписчики темы |
| `threads_get(id, ...)` | Получить тему |
| `threads_hide(...)` | Скрыть тему |
| `threads_list(...)` | Список тем |
| `threads_move(...)` | Переместить тему |
| `threads_navigation(...)` | Навигация |
| `threads_poll_get(id)` | Получить опрос |
| `threads_poll_vote(...)` | Голосовать |
| `threads_recent(...)` | Недавние темы |
| `threads_star(id)` | В избранное |
| `threads_unfollow(id)` | Отписаться |
| `threads_unread(...)` | Непрочитанные темы |
| `threads_unstar(id)` | Убрать из избранного |

</details>

<details>
<summary>👥 Users (30)</summary>

| Метод | Описание |
|---|---|
| `users_avatar_crop(...)` | Обрезать аватар |
| `users_avatar_delete(id)` | Удалить аватар |
| `users_avatar_upload(...)` | Загрузить аватар |
| `users_background_crop(...)` | Обрезать фон |
| `users_background_delete(id)` | Удалить фон |
| `users_background_upload(...)` | Загрузить фон |
| `users_claims(id)` | Жалобы пользователя |
| `users_contents(...)` | Контент пользователя |
| `users_edit(...)` | Редактировать пользователя |
| `users_fields()` | Поля пользователя |
| `users_find(...)` | Поиск пользователей |
| `users_follow(id)` | Подписаться |
| `users_followers(id, ...)` | Подписчики |
| `users_followings(id, ...)` | Подписки |
| `users_get(id, ...)` | Получить пользователя |
| `users_ignore(id)` | Игнорировать |
| `users_ignore_edit(...)` | Настройки игнора |
| `users_ignored()` | Игнорируемые |
| `users_likes(id)` | Лайки пользователя |
| `users_list(...)` | Список пользователей |
| `users_sa_cancel_reset(...)` | Отмена сброса СА |
| `users_sa_reset(...)` | Сброс секретного ответа |
| `users_secret_answer_types()` | Типы СА |
| `users_trophies(id)` | Трофеи |
| `users_unfollow(id)` | Отписаться |
| `users_unignore(id)` | Разигнорировать |

</details>

### Market API (115 эндпоинтов)

<details>
<summary>📤 Account Publishing (4)</summary>

| Метод | Описание |
|---|---|
| `publishing_add(...)` | Добавить аккаунт |
| `publishing_check(...)` | Проверить данные |
| `publishing_external(...)` | Внешний аккаунт |
| `publishing_fast_sell(...)` | Быстрая публикация |

</details>

<details>
<summary>🛒 Account Purchasing (5)</summary>

| Метод | Описание |
|---|---|
| `purchasing_check(...)` | Проверить аккаунт |
| `purchasing_confirm(...)` | Подтвердить покупку |
| `purchasing_discount_cancel(...)` | Отменить скидку |
| `purchasing_discount_request(...)` | Запросить скидку |
| `purchasing_fast_buy(...)` | Быстрая покупка |

</details>

<details>
<summary>📋 Accounts List (6)</summary>

| Метод | Описание |
|---|---|
| `list_download(...)` | Скачать данные |
| `list_favorites(...)` | Избранные аккаунты |
| `list_orders(...)` | Купленные аккаунты |
| `list_states(...)` | Статусы аккаунтов |
| `list_user(...)` | Аккаунты пользователя |
| `list_viewed(...)` | Просмотренные |

</details>

<details>
<summary>⚙️ Accounts Managing (39)</summary>

| Метод | Описание |
|---|---|
| `managing_ai_price(...)` | AI-цена |
| `managing_auto_bump(...)` | Авто-бамп |
| `managing_auto_bump_disable(...)` | Отключить авто-бамп |
| `managing_auto_buy_price(...)` | Цена авто-покупки |
| `managing_bulk_get(...)` | Массовое получение |
| `managing_bump(id)` | Бампнуть |
| `managing_change_password(...)` | Сменить пароль |
| `managing_check_guarantee(...)` | Проверить гарантию |
| `managing_close(id)` | Закрыть аккаунт |
| `managing_create_claim(...)` | Создать жалобу |
| `managing_decline_video_recording(...)` | Отклонить видео |
| `managing_edit(...)` | Редактировать |
| `managing_email_code(...)` | Email-код |
| `managing_favorite(id)` | В избранное |
| `managing_get(id)` | Получить аккаунт |
| `managing_get_letters2(...)` | Email-письма |
| `managing_image(id)` | Изображение |
| `managing_note(...)` | Заметка |
| `managing_open(id)` | Открыть аккаунт |
| `managing_public_tag(...)` | Публичный тег |
| `managing_public_untag(...)` | Удалить публичный тег |
| `managing_refuse_guarantee(...)` | Отменить гарантию |
| `managing_steam_add_mafile(...)` | Добавить Mafile |
| `managing_steam_get_mafile(id)` | Получить Mafile |
| `managing_steam_remove_mafile(id)` | Удалить Mafile |
| `managing_steam_inventory_value(id)` | Стоимость инвентаря |
| `managing_steam_mafile_code(id)` | Код Mafile |
| `managing_steam_preview(id)` | Steam HTML |
| `managing_steam_sda(...)` | Подтверждение SDA |
| `managing_steam_update_value(id)` | Обновить стоимость |
| `managing_steam_value(id)` | Стоимость Steam |
| `managing_stick(id)` | Закрепить |
| `managing_tag(...)` | Добавить тег |
| `managing_telegram_code(...)` | Telegram-код |
| `managing_telegram_reset_auth(...)` | Сброс Telegram |
| `managing_temp_email_password(...)` | Пароль temp email |
| `managing_transfer(...)` | Передать аккаунт |
| `managing_unfavorite(id)` | Убрать из избранного |
| `managing_unstick(id)` | Открепить |

</details>

<details>
<summary>📦 Batch (1)</summary>

| Метод | Описание |
|---|---|
| `batch(...)` | Пакетный запрос |

</details>

<details>
<summary>🛒 Cart (3)</summary>

| Метод | Описание |
|---|---|
| `cart_add(id)` | Добавить в корзину |
| `cart_delete(id)` | Удалить из корзины |
| `cart_get(...)` | Содержимое корзины |

</details>

<details>
<summary>📂 Categories (3)</summary>

| Метод | Описание |
|---|---|
| `category_games(name)` | Игры категории |
| `category_list(...)` | Список категорий |
| `category_params(name)` | Параметры поиска |

</details>

<details>
<summary>🔍 Category Search (25)</summary>

| Метод | Описание |
|---|---|
| `category_all(...)` | Все аккаунты |
| `category_battle_net(...)` | Battle.net |
| `category_chat_gpt(...)` | ChatGPT |
| `category_discord(...)` | Discord |
| `category_ea(...)` | EA (Origin) |
| `category_epic_games(...)` | Epic Games |
| `category_escape_from_tarkov(...)` | Escape from Tarkov |
| `category_fortnite(...)` | Fortnite |
| `category_gifts(...)` | Подарки |
| `category_hytale(...)` | Hytale |
| `category_instagram(...)` | Instagram |
| `category_mihoyo(...)` | miHoYo |
| `category_minecraft(...)` | Minecraft |
| `category_riot(...)` | Riot |
| `category_roblox(...)` | Roblox |
| `category_social_club(...)` | Social Club |
| `category_steam(...)` | Steam |
| `category_supercell(...)` | Supercell |
| `category_telegram(...)` | Telegram |
| `category_tik_tok(...)` | TikTok |
| `category_uplay(...)` | Uplay |
| `category_vpn(...)` | VPN |
| `category_warface(...)` | Warface |
| `category_wot(...)` | World of Tanks |
| `category_wot_blitz(...)` | WoT Blitz |

</details>

<details>
<summary>🏷️ Custom Discounts (4)</summary>

| Метод | Описание |
|---|---|
| `custom_discounts_create(...)` | Создать скидку |
| `custom_discounts_delete(id)` | Удалить скидку |
| `custom_discounts_edit(...)` | Редактировать скидку |
| `custom_discounts_get()` | Получить скидки |

</details>

<details>
<summary>📧 IMAP (2)</summary>

| Метод | Описание |
|---|---|
| `imap_create(...)` | Создать IMAP |
| `imap_delete(id)` | Удалить IMAP |

</details>

<details>
<summary>🧾 Invoices (3)</summary>

| Метод | Описание |
|---|---|
| `payments_invoice_create(...)` | Создать инвойс |
| `payments_invoice_get(id)` | Получить инвойс |
| `payments_invoice_list(...)` | Список инвойсов |

</details>

<details>
<summary>💰 Payments (12)</summary>

| Метод | Описание |
|---|---|
| `auto_payments_create(...)` | Создать авто-платёж |
| `auto_payments_delete(id)` | Удалить авто-платёж |
| `auto_payments_list()` | Список авто-платежей |
| `payments_balance_list()` | Список балансов |
| `payments_balance_exchange(...)` | Обмен валюты |
| `payments_cancel(...)` | Отменить перевод |
| `payments_currency()` | Получить валюту |
| `payments_fee(...)` | Комиссия перевода |
| `payments_history(...)` | История платежей |
| `payments_payout(...)` | Вывод средств |
| `payments_payout_services()` | Сервисы вывода |
| `payments_transfer(...)` | Перевод денег |

</details>

<details>
<summary>👤 Profile (2)</summary>

| Метод | Описание |
|---|---|
| `profile_edit(...)` | Редактировать профиль |
| `profile_get(...)` | Получить профиль |

</details>

<details>
<summary>🌐 Proxy (3)</summary>

| Метод | Описание |
|---|---|
| `proxy_add(...)` | Добавить прокси |
| `proxy_delete(id)` | Удалить прокси |
| `proxy_get()` | Получить прокси |

</details>

## Типизированные Enum'ы

Кодогенератор автоматически извлекает `enum`-ы из OpenAPI-схем — 23 типа с `Serialize`/`Deserialize`, `Display`, `Default` и catch-all вариантом для новых значений:

```rust
use lolzteam::models::{Currency, OrderBy, CategoryName, Premium};

// Enum'ы можно использовать напрямую в параметрах
let currency = Currency::Rub;
println!("{}", currency); // "rub"

// Неизвестные значения автоматически десериализуются в Unknown
let parsed: Currency = serde_json::from_str("\"new_currency\"").unwrap();
assert!(matches!(parsed, Currency::Unknown(_)));
```

### Список сгенерированных Enum'ов

| Enum | Описание |
|---|---|
| `RoomId` | ID комнат чата (1–13) |
| `CategoryId` | ID категорий маркета (1–29) |
| `Currency` | Валюты: `Rub`, `Uah`, `Kzt`, `Byn`, `Usd`, `Eur`, `Gbp`, `Cny`, `Try` |
| `DatePeriod` | Периоды дат: `3days`, `7days`, `14days`, `30days` |
| `OrderBy` | Сортировка: `PriceToUp`, `PriceToDown`, `Pdate`, `PdateToDown`, `PdateToUp` и др. |
| `Premium` | Типы Premium: `0`–`4` |
| `CategoryName` | Названия категорий: `Steam`, `Fortnite`, `Minecraft`, `Telegram`, `Discord` и др. |
| `Show` | Фильтры отображения: `Paid`, `Deleted`, `Recalled` и др. |
| `LastTransDatePeriod` | Периоды последней транзакции |
| `RegPeriod`, `PremiumExpirationPeriod`, `SubscriptionPeriod` | Периоды регистрации/премиума/подписки |
| `Autorenewal`, `ChangeEmail`, `Cookies`, `Email`, `Tel`, `TempEmail`, `Verified` | Фильтры маркета (yes/no) |
| `AppId` | ID приложений (730, 440, 570 и др.) |
| `NotEmailProvider` | Провайдеры email |
| `WargamingClan` | Фильтр клана Wargaming |
| `YesNoNoMatterScheme` | Тройной выбор: `yes` / `no` / `nomatter` |

## Соответствие ТЗ

✅ Генерация методов и схем ответов из OpenAPI  
✅ Forum + Market в одном крейте  
✅ Раздельные прокси для forum/market  
✅ Автоматический ретрай 429 / 502 / 503 (+ 504 и connect/timeout)  
✅ MIT лицензия  
✅ GitHub Actions CI/CD  

## Лицензия

MIT


