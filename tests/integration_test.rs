// Live API тесты. Только GET-запросы, ничего не покупает/создаёт/удаляет.
//
// LOLZTEAM_TOKEN=xxx cargo test -- --test-threads=1

use lolzteam::LolzteamClient;
use lolzteam::forum::types::*;
use lolzteam::market::types::*;

fn token() -> String {
    std::env::var("LOLZTEAM_TOKEN").expect("set LOLZTEAM_TOKEN")
}

fn client() -> LolzteamClient {
    LolzteamClient::new(token())
}

fn has_key(v: &serde_json::Value, key: &str) {
    assert!(
        v.get(key).is_some(),
        "no key '{key}' in: {}",
        serde_json::to_string_pretty(v).unwrap_or_default()
    );
}

// --- forum ---

#[tokio::test]
async fn forum_users_get() {
    let resp = client().forum().users_get(1, None).await.unwrap();
    has_key(&resp, "user");
    has_key(&resp, "system_info");
    assert!(resp["user"]["user_id"].as_i64().unwrap() > 0);
    assert!(resp["user"]["username"].as_str().is_some());
}

#[tokio::test]
async fn forum_users_find() {
    let resp = client()
        .forum()
        .users_find(Some("AS7RIDENIED".into()), None, None)
        .await
        .unwrap();
    assert!(resp.get("users").is_some() || resp.get("exact").is_some());
}

#[tokio::test]
async fn forum_users_list() {
    match client().forum().users_list(Some(1), Some(3), None).await {
        Ok(data) => has_key(&data, "users"),
        Err(lolzteam::Error::Api { status: 403, .. }) => {
            // не хватает прав — ок
        }
        Err(e) => panic!("{e}"),
    }
}

#[tokio::test]
async fn forum_users_followers() {
    let resp = client().forum().users_followers(1, None, None, None).await.unwrap();
    has_key(&resp, "users");
}

#[tokio::test]
async fn forum_users_trophies() {
    let resp = client().forum().users_trophies(1).await.unwrap();
    has_key(&resp, "system_info");
}

#[tokio::test]
async fn forum_categories_list() {
    let resp = client().forum().categories_list(None, None, None).await.unwrap();
    has_key(&resp, "categories");
    let cats = resp["categories"].as_array().unwrap();
    assert!(!cats.is_empty());
}

#[tokio::test]
async fn forum_forums_list() {
    let resp = client().forum().forums_list(None, None, None).await.unwrap();
    has_key(&resp, "forums");
    let forums = resp["forums"].as_array().unwrap();
    assert!(!forums.is_empty());
    assert!(forums[0]["forum_id"].as_i64().is_some());
}

#[tokio::test]
async fn forum_forums_grouped() {
    let resp = client().forum().forums_grouped().await.unwrap();
    has_key(&resp, "system_info");
}

#[tokio::test]
async fn forum_threads_list() {
    let resp = client()
        .forum()
        .threads_list(ForumThreadsListParams {
            limit: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
    has_key(&resp, "threads");
    assert!(!resp["threads"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn forum_threads_get() {
    let c = client();
    let list = c
        .forum()
        .threads_list(ForumThreadsListParams {
            limit: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();

    let tid = list["threads"][0]["thread_id"].as_i64().unwrap();
    let resp = c.forum().threads_get(tid, None).await.unwrap();
    has_key(&resp, "thread");
    assert_eq!(resp["thread"]["thread_id"].as_i64().unwrap(), tid);
}

#[tokio::test]
async fn forum_threads_recent() {
    let resp = client()
        .forum()
        .threads_recent(ForumThreadsRecentParams {
            days: Some(7),
            limit: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
    has_key(&resp, "threads");
}

#[tokio::test]
async fn forum_posts_list() {
    let c = client();
    let list = c
        .forum()
        .threads_list(ForumThreadsListParams {
            limit: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();

    let tid = list["threads"][0]["thread_id"].as_i64().unwrap();
    let resp = c
        .forum()
        .posts_list(ForumPostsListParams {
            thread_id: Some(tid),
            limit: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
    has_key(&resp, "posts");
}

#[tokio::test]
async fn forum_notifications_list() {
    let resp = client().forum().notifications_list(None, None, Some(3)).await.unwrap();
    has_key(&resp, "notifications");
    has_key(&resp, "system_info");
}

#[tokio::test]
async fn forum_conversations_list() {
    match client().forum().conversations_list(None, None, Some(3)).await {
        Ok(data) => has_key(&data, "conversations"),
        Err(lolzteam::Error::Api { status: 403, .. }) => {}
        Err(e) => panic!("{e}"),
    }
}

#[tokio::test]
async fn forum_navigation_list() {
    match client().forum().navigation_list(None).await {
        Ok(data) => has_key(&data, "elements"),
        Err(lolzteam::Error::Api { status, .. }) if status == 403 || status == 500 => {
            // серверная ошибка — бывает
        }
        Err(e) => panic!("{e}"),
    }
}

#[tokio::test]
async fn forum_tags_popular() {
    let resp = client().forum().tags_popular().await.unwrap();
    has_key(&resp, "tags");
}

#[tokio::test]
async fn forum_chatbox_index() {
    let resp = client().forum().chatbox_index(None).await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn forum_feed_options() {
    let resp = client().forum().forums_get_feed_options().await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn forum_user_fields() {
    let resp = client().forum().users_fields().await.unwrap();
    assert!(resp.is_object());
}

// --- market ---

#[tokio::test]
async fn market_category_list() {
    let resp = client().market().category_list(None).await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn market_category_all() {
    let resp = client()
        .market()
        .category_all(MarketCategoryAllParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    has_key(&resp, "items");
    has_key(&resp, "totalItems");
}

#[tokio::test]
async fn market_category_steam() {
    let resp = client()
        .market()
        .category_steam(MarketCategorySteamParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    has_key(&resp, "items");
}

#[tokio::test]
async fn market_category_telegram() {
    let resp = client()
        .market()
        .category_telegram(MarketCategoryTelegramParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    has_key(&resp, "items");
}

#[tokio::test]
async fn market_category_discord() {
    let resp = client()
        .market()
        .category_discord(MarketCategoryDiscordParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    has_key(&resp, "items");
}

#[tokio::test]
async fn market_category_fortnite() {
    let resp = client()
        .market()
        .category_fortnite(MarketCategoryFortniteParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    has_key(&resp, "items");
}

#[tokio::test]
async fn market_profile_get() {
    let resp = client().market().profile_get(None).await.unwrap();
    has_key(&resp, "user");
    assert!(resp["user"]["user_id"].as_i64().is_some());
    assert!(resp["user"]["username"].as_str().is_some());
}

#[tokio::test]
async fn market_payments_currency() {
    let resp = client().market().payments_currency().await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn market_payments_history() {
    let resp = client()
        .market()
        .payments_history(MarketPaymentsHistoryParams::default())
        .await
        .unwrap();
    has_key(&resp, "payments");
    has_key(&resp, "system_info");
}

#[tokio::test]
async fn market_list_favorites() {
    let resp = client()
        .market()
        .list_favorites(MarketListFavoritesParams::default())
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_list_orders() {
    let resp = client()
        .market()
        .list_orders(MarketListOrdersParams::default())
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_list_user_items() {
    let resp = client()
        .market()
        .list_user(MarketListUserParams::default())
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_list_viewed() {
    let resp = client()
        .market()
        .list_viewed(MarketListViewedParams::default())
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_cart_get() {
    let resp = client()
        .market()
        .cart_get(MarketCartGetParams::default())
        .await
        .unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn market_category_params() {
    let resp = client().market().category_params("steam".into()).await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn market_category_games() {
    let resp = client().market().category_games("steam".into()).await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn market_proxy_get() {
    let resp = client().market().proxy_get().await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn market_list_states() {
    let resp = client().market().list_states(None).await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn market_balance_list() {
    let resp = client().market().payments_balance_list().await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn market_custom_discounts() {
    let resp = client().market().custom_discounts_get().await.unwrap();
    assert!(resp.is_object());
}

#[tokio::test]
async fn market_auto_payments() {
    let resp = client().market().auto_payments_list().await.unwrap();
    assert!(resp.is_object());
}

// --- builder (offline) ---

#[test]
fn client_default() {
    let c = LolzteamClient::new("test");
    let _ = c.forum();
    let _ = c.market();
}

#[test]
fn client_with_proxy() {
    let r = LolzteamClient::builder("test")
        .proxy("socks5://127.0.0.1:1080")
        .max_retries(3)
        .timeout(std::time::Duration::from_secs(10))
        .build();
    assert!(r.is_ok());
}

#[test]
fn client_separate_proxies() {
    let r = LolzteamClient::builder("test")
        .forum_proxy("http://proxy1.example.com:8080")
        .market_proxy("socks5://proxy2.example.com:1080")
        .build();
    assert!(r.is_ok());
}
