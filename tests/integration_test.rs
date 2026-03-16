// Live API тесты. Только GET-запросы, ничего не покупает/создаёт/удаляет.
//
// LOLZTEAM_TOKEN=xxx cargo test -- --test-threads=1

use lolzteam::forum::types::*;
use lolzteam::market::types::*;
use lolzteam::LolzteamClient;

fn token() -> String {
    std::env::var("LOLZTEAM_TOKEN").expect("set LOLZTEAM_TOKEN")
}

fn client() -> LolzteamClient {
    LolzteamClient::new(token())
}

fn has_token() -> bool {
    std::env::var("LOLZTEAM_TOKEN").is_ok()
}

/// Skip test when LOLZTEAM_TOKEN is not set.
macro_rules! require_token {
    () => {
        if !has_token() {
            eprintln!("skipped: LOLZTEAM_TOKEN not set");
            return;
        }
    };
}

// --- forum ---

#[tokio::test]
async fn forum_users_get() {
    require_token!();
    let resp = client().forum().users_get(1, None).await.unwrap();
    assert!(resp.user.user_id > 0);
    assert!(!resp.user.username.is_empty());
}

#[tokio::test]
async fn forum_users_find() {
    require_token!();
    let _resp = client()
        .forum()
        .users_find(Some("AS7RIDENIED".into()), None, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn forum_users_list() {
    require_token!();
    match client().forum().users_list(Some(1), Some(3), None).await {
        Ok(_data) => {}
        Err(lolzteam::Error::Api { status: 403, .. }) => {}
        Err(e) => panic!("{e}"),
    }
}

#[tokio::test]
async fn forum_users_followers() {
    require_token!();
    let _resp = client()
        .forum()
        .users_followers(1, None, None, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn forum_users_trophies() {
    require_token!();
    let resp = client().forum().users_trophies(1).await.unwrap();
    let _ = &resp.system_info;
}

#[tokio::test]
async fn forum_categories_list() {
    require_token!();
    let resp = client()
        .forum()
        .categories_list(None, None, None)
        .await
        .unwrap();
    assert!(!resp.categories.is_empty());
}

#[tokio::test]
async fn forum_forums_list() {
    require_token!();
    let resp = client()
        .forum()
        .forums_list(None, None, None)
        .await
        .unwrap();
    assert!(!resp.forums.is_empty());
}

#[tokio::test]
async fn forum_forums_grouped() {
    require_token!();
    let resp = client().forum().forums_grouped().await.unwrap();
    let _ = &resp.system_info;
}

#[tokio::test]
async fn forum_threads_list() {
    require_token!();
    let resp = client()
        .forum()
        .threads_list(ForumThreadsListParams {
            limit: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(!resp.threads.is_empty());
}

#[tokio::test]
async fn forum_threads_get() {
    require_token!();
    let c = client();
    let list = c
        .forum()
        .threads_list(ForumThreadsListParams {
            limit: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();

    let tid = list.threads[0].thread_id;
    let resp = c.forum().threads_get(tid, None).await.unwrap();
    assert_eq!(resp.thread.thread_id, tid);
}

#[tokio::test]
async fn forum_threads_recent() {
    require_token!();
    let _resp = client()
        .forum()
        .threads_recent(ForumThreadsRecentParams {
            days: Some(7),
            limit: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn forum_posts_list() {
    require_token!();
    let c = client();
    let list = c
        .forum()
        .threads_list(ForumThreadsListParams {
            limit: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();

    let tid = list.threads[0].thread_id;
    let _resp = c
        .forum()
        .posts_list(ForumPostsListParams {
            thread_id: Some(tid),
            limit: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn forum_notifications_list() {
    require_token!();
    let resp = client()
        .forum()
        .notifications_list(None, None, Some(3))
        .await
        .unwrap();
    let _ = &resp.system_info;
}

#[tokio::test]
async fn forum_conversations_list() {
    require_token!();
    match client()
        .forum()
        .conversations_list(None, None, Some(3))
        .await
    {
        Ok(_data) => {}
        Err(lolzteam::Error::Api { status: 403, .. }) => {}
        Err(e) => panic!("{e}"),
    }
}

#[tokio::test]
async fn forum_navigation_list() {
    require_token!();
    match client().forum().navigation_list(None).await {
        Ok(_data) => {}
        Err(lolzteam::Error::Api { status, .. }) if status == 403 || status == 500 => {}
        Err(e) => panic!("{e}"),
    }
}

#[tokio::test]
async fn forum_tags_popular() {
    require_token!();
    let _resp = client().forum().tags_popular().await.unwrap();
}

#[tokio::test]
async fn forum_chatbox_index() {
    require_token!();
    let _resp = client().forum().chatbox_index(None).await.unwrap();
}

#[tokio::test]
async fn forum_feed_options() {
    require_token!();
    let _resp = client().forum().forums_get_feed_options().await.unwrap();
}

#[tokio::test]
async fn forum_user_fields() {
    require_token!();
    let _resp = client().forum().users_fields().await.unwrap();
}

// --- market ---

#[tokio::test]
async fn market_category_list() {
    require_token!();
    let _resp = client().market().category_list(None).await.unwrap();
}

#[tokio::test]
async fn market_category_all() {
    require_token!();
    let _resp = client()
        .market()
        .category_all(MarketCategoryAllParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn market_category_steam() {
    require_token!();
    let _resp = client()
        .market()
        .category_steam(MarketCategorySteamParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn market_category_telegram() {
    require_token!();
    let _resp = client()
        .market()
        .category_telegram(MarketCategoryTelegramParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn market_category_discord() {
    require_token!();
    let _resp = client()
        .market()
        .category_discord(MarketCategoryDiscordParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn market_category_fortnite() {
    require_token!();
    let _resp = client()
        .market()
        .category_fortnite(MarketCategoryFortniteParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn market_profile_get() {
    require_token!();
    let resp = client().market().profile_get(None).await.unwrap();
    assert!(resp.user.user_id > 0);
    assert!(!resp.user.username.is_empty());
}

#[tokio::test]
async fn market_payments_currency() {
    require_token!();
    let _resp = client().market().payments_currency().await.unwrap();
}

#[tokio::test]
async fn market_payments_history() {
    require_token!();
    let _resp = client()
        .market()
        .payments_history(MarketPaymentsHistoryParams::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn market_list_favorites() {
    require_token!();
    let resp = client()
        .market()
        .list_favorites(MarketListFavoritesParams::default())
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_list_orders() {
    require_token!();
    let resp = client()
        .market()
        .list_orders(MarketListOrdersParams::default())
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_list_user_items() {
    require_token!();
    let resp = client()
        .market()
        .list_user(MarketListUserParams::default())
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_list_viewed() {
    require_token!();
    let resp = client()
        .market()
        .list_viewed(MarketListViewedParams::default())
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_cart_get() {
    require_token!();
    let _resp = client()
        .market()
        .cart_get(MarketCartGetParams::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn market_category_params() {
    require_token!();
    let _resp = client()
        .market()
        .category_params("steam".into())
        .await
        .unwrap();
}

#[tokio::test]
async fn market_category_games() {
    require_token!();
    let _resp = client()
        .market()
        .category_games("steam".into())
        .await
        .unwrap();
}

#[tokio::test]
async fn market_proxy_get() {
    require_token!();
    let _resp = client().market().proxy_get().await.unwrap();
}

#[tokio::test]
async fn market_list_states() {
    require_token!();
    let _resp = client().market().list_states(None).await.unwrap();
}

#[tokio::test]
async fn market_balance_list() {
    require_token!();
    let _resp = client().market().payments_balance_list().await.unwrap();
}

#[tokio::test]
async fn market_custom_discounts() {
    require_token!();
    let _resp = client().market().custom_discounts_get().await.unwrap();
}

#[tokio::test]
async fn market_auto_payments() {
    require_token!();
    let _resp = client().market().auto_payments_list().await.unwrap();
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
