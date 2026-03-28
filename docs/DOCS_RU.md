# Документация — lolzteam

Полный справочник методов Rust-клиента для LOLZTEAM Forum и Market API.

## Настройка клиента

| Метод | Описание | По умолчанию |
|---|---|---|
| `LolzteamClient::new(token)` | Создать клиент с настройками по умолчанию | `Result<Self>` |
| `.builder(token)` | Создать билдер для тонкой настройки | — |
| `.proxy(url)` | Общий прокси для Forum и Market | Нет |
| `.forum_proxy(url)` | Прокси только для Forum API | Нет |
| `.market_proxy(url)` | Прокси только для Market API | Нет |
| `.max_retries(n)` | Максимум попыток при ретрае | 5 |
| `.timeout(duration)` | Таймаут запроса | 30 сек |
| `.forum_base_url(url)` | Кастомный URL Forum API | `https://prod-api.lolz.live` |
| `.market_base_url(url)` | Кастомный URL Market API | `https://prod-api.lzt.market` |

## Типы ошибок

| Тип | Описание |
|---|---|
| `Error::Http` | Сетевая ошибка (таймаут, DNS, соединение) |
| `Error::Json` | Ошибка парсинга JSON-ответа |
| `Error::Auth` | 401 Unauthorized |
| `Error::Forbidden` | 403 Forbidden |
| `Error::NotFound` | 404 Not Found |
| `Error::RateLimited` | Исчерпаны все попытки ретрая |
| `Error::Api` | Прочие HTTP-ошибки от API |

## Кодогенерация

Сгенерированные файлы:
- `src/models.rs` — модели ответов (общие для Forum и Market)
- `src/forum/types.rs` — параметры запросов Forum API
- `src/forum/methods.rs` — методы Forum API
- `src/market/types.rs` — параметры запросов Market API
- `src/market/methods.rs` — методы Market API

Перегенерация:
```bash
make generate
```

---


## Тела запросов

- `RequestBody::Json(...)` — JSON-полезная нагрузка
- `RequestBody::Form(...)` — `application/x-www-form-urlencoded`
- `RequestBody::Multipart(...)` — multipart upload через `MultipartForm` и `MultipartFile`

Пример загрузки аватара:

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

## Параметры эндпоинтов

Эндпоинты с более чем тремя необязательными параметрами принимают структуру `*Params`.

```rust
use lolzteam::market::types::MarketCategorySteamParams;

let params = MarketCategorySteamParams {
    pmin: Some(10),
    pmax: Some(100),
    ..Default::default()
};
let results = client.market().category_steam(params).await?;
```

## Forum API

### Categories
| Метод | Описание |
|---|---|
| `categories_get(category_id)` | Получить категорию |
| `categories_list(parent_category_id, parent_forum_id, order)` | Список категорий |

### Forums
| Метод | Описание |
|---|---|
| `forums_get(forum_id)` | Получить форум |
| `forums_list(parent_category_id, parent_forum_id, order)` | Список форумов |
| `forums_grouped()` | Форумы, сгруппированные по категориям |
| `forums_follow(params)` | Подписаться на форум |
| `forums_unfollow(forum_id)` | Отписаться от форума |
| `forums_followed(total)` | Список подписок на форумы |
| `forums_followers(forum_id)` | Подписчики форума |
| `forums_get_feed_options()` | Настройки ленты |
| `forums_edit_feed_options(...)` | Изменить настройки ленты |

### Threads
| Метод | Описание |
|---|---|
| `threads_list(params)` | Список тем |
| `threads_get(thread_id, ...)` | Получить тему |
| `threads_create(params)` | Создать тему |
| `threads_edit(params)` | Редактировать тему |
| `threads_delete(thread_id, ...)` | Удалить тему |
| `threads_move(thread_id, params)` | Переместить тему |
| `threads_bump(thread_id)` | Поднять тему |
| `threads_recent(params)` | Недавние темы |
| `threads_unread(...)` | Непрочитанные темы |
| `threads_create_contest(params)` | Создать конкурс |
| `threads_claim(params)` | Создать жалобу |
| `threads_follow(thread_id, ...)` | Подписаться на тему |
| `threads_unfollow(thread_id)` | Отписаться от темы |
| `threads_followed(...)` | Подписки на темы |
| `threads_followers(thread_id)` | Подписчики темы |
| `threads_poll_get(thread_id)` | Получить опрос |
| `threads_poll_vote(thread_id, ...)` | Проголосовать |
| `threads_star(thread_id)` | Добавить в избранное |
| `threads_unstar(thread_id)` | Убрать из избранного |
| `threads_hide(thread_id)` | Скрыть тему |
| `threads_finish(thread_id)` | Завершить тему |
| `threads_navigation(thread_id)` | Навигация по теме |

### Posts
| Метод | Описание |
|---|---|
| `posts_list(params)` | Список постов |
| `posts_get(post_id)` | Получить пост |
| `posts_create(thread_id, post_body)` | Создать пост |
| `posts_edit(post_id, ...)` | Редактировать пост |
| `posts_delete(post_id, ...)` | Удалить пост |
| `posts_like(post_id)` | Лайкнуть пост |
| `posts_unlike(post_id)` | Убрать лайк |
| `posts_likes(post_id, ...)` | Список лайков |
| `posts_report(post_id, message)` | Пожаловаться на пост |
| `posts_report_reasons(post_id)` | Причины жалобы |
| `posts_comments_create(...)` | Создать комментарий к посту |
| `posts_comments_get(...)` | Получить комментарий |
| `posts_comments_edit(...)` | Редактировать комментарий |
| `posts_comments_delete(...)` | Удалить комментарий |

### Profile Posts
| Метод | Описание |
|---|---|
| `profile_posts_list(params)` | Список постов профиля |
| `profile_posts_get(profile_post_id)` | Получить пост профиля |
| `profile_posts_create(...)` | Создать пост профиля |
| `profile_posts_edit(...)` | Редактировать пост профиля |
| `profile_posts_delete(...)` | Удалить пост профиля |
| `profile_posts_like(profile_post_id)` | Лайкнуть |
| `profile_posts_unlike(profile_post_id)` | Убрать лайк |
| `profile_posts_stick(profile_post_id)` | Закрепить |
| `profile_posts_unstick(profile_post_id)` | Открепить |
| `profile_posts_comments_list(...)` | Комментарии к посту профиля |
| `profile_posts_comments_create(...)` | Создать комментарий |
| `profile_posts_comments_edit(...)` | Редактировать комментарий |
| `profile_posts_comments_delete(...)` | Удалить комментарий |

### Users
| Метод | Описание |
|---|---|
| `users_get(user_id, ...)` | Получить пользователя |
| `users_find(username, ...)` | Найти пользователя |
| `users_list(page, limit, ...)` | Список пользователей |
| `users_edit(user_id, params)` | Редактировать профиль |
| `users_follow(user_id)` | Подписаться |
| `users_unfollow(user_id)` | Отписаться |
| `users_followers(user_id, ...)` | Подписчики |
| `users_followings(user_id, ...)` | Подписки |
| `users_ignore(user_id)` | Игнорировать |
| `users_unignore(user_id)` | Убрать из игнора |
| `users_ignored(total)` | Список игнорируемых |
| `users_likes(user_id, params)` | Лайки пользователя |
| `users_trophies(user_id)` | Трофеи |
| `users_contents(user_id, ...)` | Контент пользователя |
| `users_claims(user_id, ...)` | Жалобы пользователя |
| `users_fields()` | Поля профиля |
| `users_avatar_upload(user_id, params)` | Загрузить аватар |
| `users_avatar_delete(user_id)` | Удалить аватар |
| `users_background_upload(user_id, params)` | Загрузить фон |
| `users_background_delete(user_id)` | Удалить фон |

### Conversations
| Метод | Описание |
|---|---|
| `conversations_list(page, limit, ...)` | Список диалогов |
| `conversations_get(conversation_id)` | Получить диалог |
| `conversations_create(params)` | Создать диалог |
| `conversations_update(params)` | Обновить диалог |
| `conversations_delete(conversation_id)` | Удалить диалог |
| `conversations_read(conversation_id)` | Отметить прочитанным |
| `conversations_read_all()` | Отметить все прочитанными |
| `conversations_messages_list(conversation_id, params)` | Сообщения диалога |
| `conversations_messages_get(...)` | Получить сообщение |
| `conversations_messages_create(...)` | Отправить сообщение |
| `conversations_messages_edit(...)` | Редактировать сообщение |
| `conversations_messages_delete(...)` | Удалить сообщение |
| `conversations_search(...)` | Поиск по диалогам |
| `conversations_star(conversation_id)` | В избранное |
| `conversations_unstar(conversation_id)` | Из избранного |
| `conversations_invite(...)` | Пригласить в диалог |
| `conversations_kick(...)` | Исключить из диалога |

### Search
| Метод | Описание |
|---|---|
| `search_all(params)` | Поиск по всему |
| `search_threads(params)` | Поиск тем |
| `search_posts(params)` | Поиск постов |
| `search_profile_posts(params)` | Поиск постов профиля |
| `search_tagged(params)` | Поиск по тегам |
| `search_users(q)` | Поиск пользователей |
| `search_results(search_id, ...)` | Результаты поиска |

### Notifications
| Метод | Описание |
|---|---|
| `notifications_list(page, limit, ...)` | Список уведомлений |
| `notifications_get(notification_id)` | Получить уведомление |
| `notifications_read(notification_id)` | Отметить прочитанным |

### Tags
| Метод | Описание |
|---|---|
| `tags_popular()` | Популярные теги |
| `tags_list(page, limit)` | Список тегов |
| `tags_get(tag_id)` | Получить тег |
| `tags_find(tag)` | Найти тег |

### Chatbox
| Метод | Описание |
|---|---|
| `chatbox_index(...)` | Главная чатбокса |
| `chatbox_get_messages(...)` | Сообщения чатбокса |
| `chatbox_post_message(...)` | Отправить сообщение |
| `chatbox_edit_message(...)` | Редактировать сообщение |
| `chatbox_delete_message(message_id)` | Удалить сообщение |
| `chatbox_online(...)` | Онлайн пользователи |
| `chatbox_get_leaderboard(...)` | Лидерборд |
| `chatbox_report(...)` | Пожаловаться |
| `chatbox_report_reasons()` | Причины жалобы |
| `chatbox_get_ignore()` | Список игнорируемых |
| `chatbox_post_ignore(...)` | Добавить в игнор |
| `chatbox_delete_ignore(...)` | Убрать из игнора |

### Прочее
| Метод | Описание |
|---|---|
| `navigation_list(parent)` | Навигация |
| `o_auth_token()` | Получить токен |
| `batch_execute()` | Batch-запрос |
| `assets_css(css)` | Получить CSS |
| `forms_list(page)` | Список форм |
| `forms_create()` | Создать форму |
| `pages_list(...)` | Список страниц |
| `pages_get(page_id)` | Получить страницу |
| `links_list()` | Список ссылок |
| `links_get(link_id)` | Получить ссылку |

---

## Market API

### Categories
| Метод | Описание |
|---|---|
| `category_list(top_queries)` | Список категорий |
| `category_params(category_name)` | Параметры категории |
| `category_games(category_name)` | Игры категории |

### Category Search
| Метод | Описание |
|---|---|
| `category_all(params)` | Поиск по всем категориям |
| `category_steam(params)` | Steam аккаунты |
| `category_fortnite(params)` | Fortnite аккаунты |
| `category_discord(params)` | Discord аккаунты |
| `category_telegram(params)` | Telegram аккаунты |
| `category_minecraft(params)` | Minecraft аккаунты |
| `category_roblox(params)` | Roblox аккаунты |
| `category_riot(params)` | Riot Games аккаунты |
| `category_epic_games(params)` | Epic Games аккаунты |
| `category_ea(params)` | EA аккаунты |
| `category_battle_net(params)` | Battle.net аккаунты |
| `category_uplay(params)` | Uplay аккаунты |
| `category_social_club(params)` | Social Club аккаунты |
| `category_wot(params)` | World of Tanks аккаунты |
| `category_wot_blitz(params)` | WoT Blitz аккаунты |
| `category_warface(params)` | Warface аккаунты |
| `category_supercell(params)` | Supercell аккаунты |
| `category_mihoyo(params)` | miHoYo аккаунты |
| `category_escape_from_tarkov(params)` | Escape from Tarkov аккаунты |
| `category_instagram(params)` | Instagram аккаунты |
| `category_tik_tok(params)` | TikTok аккаунты |
| `category_chat_gpt(params)` | ChatGPT аккаунты |
| `category_vpn(params)` | VPN аккаунты |
| `category_gifts(params)` | Подарочные карты |
| `category_hytale(params)` | Hytale аккаунты |

### Account Lists
| Метод | Описание |
|---|---|
| `list_user(params)` | Мои аккаунты |
| `list_favorites(params)` | Избранные |
| `list_orders(params)` | Заказы |
| `list_viewed(params)` | Просмотренные |
| `list_states(user_id)` | Статусы аккаунтов |
| `list_download(params)` | Скачать список |

### Account Managing
| Метод | Описание |
|---|---|
| `managing_get(item_id)` | Получить аккаунт |
| `managing_edit(params)` | Редактировать аккаунт |
| `manging_delete(item_id, reason)` | Удалить аккаунт |
| `managing_bump(item_id)` | Поднять аккаунт |
| `managing_auto_bump(item_id, hour)` | Автоподнятие |
| `managing_auto_bump_disable(item_id)` | Отключить автоподнятие |
| `managing_auto_buy_price(params)` | Цена автопокупки |
| `managing_stick(item_id)` | Закрепить |
| `managing_unstick(item_id)` | Открепить |
| `managing_favorite(item_id)` | В избранное |
| `managing_unfavorite(item_id)` | Из избранного |
| `managing_tag(item_id, tag_id)` | Добавить тег |
| `managing_untag(item_id, tag_id)` | Убрать тег |
| `managing_public_tag(params)` | Публичный тег |
| `managing_public_untag(params)` | Убрать публичный тег |
| `managing_note(params)` | Заметка |
| `managing_image(params)` | Изображение |
| `managing_close(item_id)` | Закрыть |
| `managing_open(item_id)` | Открыть |
| `managing_transfer(params)` | Передать аккаунт |
| `managing_change_password(params)` | Сменить пароль |
| `managing_ai_price(item_id)` | AI-оценка цены |
| `managing_bulk_get(params)` | Массовое получение |
| `managing_create_claim(params)` | Создать жалобу |
| `managing_check_guarantee(item_id)` | Проверить гарантию |
| `managing_refuse_guarantee(item_id)` | Отказаться от гарантии |
| `managing_email_code(item_id)` | Код с email |
| `managing_telegram_code(params)` | Код из Telegram |
| `managing_temp_email_password(params)` | Пароль временной почты |
| `managing_steam_value(params)` | Стоимость Steam |
| `managing_steam_update_value(params)` | Обновить стоимость Steam |
| `managing_steam_inventory_value(params)` | Стоимость инвентаря |
| `managing_steam_preview(params)` | Превью Steam |
| `managing_steam_mafile_code(item_id)` | Код из maFile |
| `managing_steam_get_mafile(params)` | Получить maFile |
| `managing_steam_add_mafile(item_id)` | Добавить maFile |
| `managing_steam_remove_mafile(item_id)` | Удалить maFile |
| `managing_steam_sda(params)` | Steam Desktop Auth |

### Account Publishing
| Метод | Описание |
|---|---|
| `publishing_add(params)` | Добавить аккаунт |
| `publishing_check(params)` | Проверить аккаунт |
| `publishing_fast_sell(params)` | Быстрая продажа |
| `publishing_external(params)` | Внешний аккаунт |

### Account Purchasing
| Метод | Описание |
|---|---|
| `purchasing_fast_buy(params)` | Быстрая покупка |
| `purchasing_confirm(params)` | Подтвердить покупку |
| `purchasing_check(item_id)` | Проверить покупку |
| `purchasing_discount_request(params)` | Запросить скидку |
| `purchasing_discount_cancel(item_id)` | Отменить запрос скидки |

### Cart
| Метод | Описание |
|---|---|
| `cart_get(params)` | Получить корзину |
| `cart_add(item_id)` | Добавить в корзину |
| `cart_delete(params)` | Удалить из корзины |

### Payments
| Метод | Описание |
|---|---|
| `payments_history(params)` | История платежей |
| `payments_currency()` | Курсы валют |
| `payments_fee(amount)` | Комиссия |
| `payments_transfer(params)` | Перевод |
| `payments_payout(params)` | Вывод средств |
| `payments_payout_services()` | Сервисы вывода |
| `payments_cancel(payment_id)` | Отменить платёж |
| `payments_balance_list()` | Список балансов |
| `payments_balance_exchange(amount, from, to)` | Обмен валют |

### Invoices
| Метод | Описание |
|---|---|
| `payments_invoice_create(params)` | Создать инвойс |
| `payments_invoice_get(invoice_id, payment_id)` | Получить инвойс |
| `payments_invoice_list(params)` | Список инвойсов |

### Auto Payments
| Метод | Описание |
|---|---|
| `auto_payments_list()` | Список автоплатежей |
| `auto_payments_create(params)` | Создать автоплатёж |
| `auto_payments_delete(auto_payment_id)` | Удалить автоплатёж |

### Custom Discounts
| Метод | Описание |
|---|---|
| `custom_discounts_get()` | Получить скидки |
| `custom_discounts_create(params)` | Создать скидку |
| `custom_discounts_edit(params)` | Редактировать скидку |
| `custom_discounts_delete(discount_id)` | Удалить скидку |

### Profile
| Метод | Описание |
|---|---|
| `profile_get(fields_include)` | Получить профиль |
| `profile_edit(params)` | Редактировать профиль |
| `profile_claims(params)` | Жалобы профиля |

### Proxy
| Метод | Описание |
|---|---|
| `proxy_get()` | Получить прокси |
| `proxy_add(params)` | Добавить прокси |
| `proxy_delete(delete_all, proxy_id)` | Удалить прокси |

### IMAP
| Метод | Описание |
|---|---|
| `imap_create(params)` | Создать IMAP |
| `imap_delete(domain)` | Удалить IMAP |

### Прочее
| Метод | Описание |
|---|---|
| `batch()` | Batch-запрос |


