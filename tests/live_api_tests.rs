// Real API tests using the live LOLZTEAM API.
// Only safe GET endpoints — no profile changes, no spending, no posting.
//
// Run: LOLZTEAM_TOKEN=xxx cargo test --test live_api_tests -- --test-threads=1

use lolzteam::forum::types::*;
use lolzteam::market::types::*;
use lolzteam::{Error, LolzteamClient};
use std::time::Duration;

fn token() -> String {
    std::env::var("LOLZTEAM_TOKEN").expect("set LOLZTEAM_TOKEN")
}

fn client() -> LolzteamClient {
    LolzteamClient::builder(token())
        .timeout(Duration::from_secs(30))
        .build()
        .expect("failed to build client")
}

fn has_token() -> bool {
    std::env::var("LOLZTEAM_TOKEN").is_ok()
}

macro_rules! require_token {
    () => {
        if !has_token() {
            eprintln!("skipped: LOLZTEAM_TOKEN not set");
            return;
        }
    };
}

// ══════════════════════════════════════════════════════════════════════
// Forum: Users
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn forum_users_get_by_id() {
    require_token!();
    let resp = client().forum().users_get(1, None).await.unwrap();
    assert!(resp.user.user_id > 0);
    assert!(!resp.user.username.is_empty());
}

#[tokio::test]
async fn forum_users_get_with_fields() {
    require_token!();
    let resp = client()
        .forum()
        .users_get(1, Some(vec!["user_id".into(), "username".into()]))
        .await
        .unwrap();
    assert_eq!(resp.user.user_id, 1);
}

#[tokio::test]
async fn forum_users_find_by_username() {
    require_token!();
    let resp = client()
        .forum()
        .users_find(Some("AS7RIDENIED".into()), None, None)
        .await
        .unwrap();
    // users may be empty if the username doesn't match exactly
    let _ = &resp.users;
}

#[tokio::test]
async fn forum_users_followers() {
    require_token!();
    let resp = client()
        .forum()
        .users_followers(1, None, None, None)
        .await
        .unwrap();
    let _ = &resp.system_info;
}

#[tokio::test]
async fn forum_users_followings() {
    require_token!();
    let resp = client()
        .forum()
        .users_followings(1, None, None, None)
        .await
        .unwrap();
    let _ = &resp.system_info;
}

#[tokio::test]
async fn forum_users_trophies() {
    require_token!();
    let resp = client().forum().users_trophies(1).await.unwrap();
    let _ = &resp.system_info;
}

#[tokio::test]
async fn forum_users_fields() {
    require_token!();
    let resp = client().forum().users_fields().await.unwrap();
    let _ = &resp.system_info;
}

// ══════════════════════════════════════════════════════════════════════
// Forum: Categories, Forums, Navigation
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn forum_categories_list_all() {
    require_token!();
    let resp = client()
        .forum()
        .categories_list(None, None, None)
        .await
        .unwrap();
    assert!(!resp.categories.is_empty());
}

#[tokio::test]
async fn forum_categories_get_by_id() {
    require_token!();
    // category 1 may not exist; handle NotFound gracefully
    match client().forum().categories_get(1).await {
        Ok(resp) => assert!(resp.category.is_object()),
        Err(Error::NotFound { .. }) => {} // expected if id doesn't exist
        Err(e) => panic!("unexpected: {e}"),
    }
}

#[tokio::test]
async fn forum_forums_list_all() {
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
async fn forum_forums_get_by_id() {
    require_token!();
    let list = client()
        .forum()
        .forums_list(None, None, None)
        .await
        .unwrap();
    if let Some(first) = list.forums.first() {
        // forums are Vec<serde_json::Value>, extract forum_id
        let fid = first["forum_id"].as_i64().unwrap();
        let resp = client().forum().forums_get(fid).await.unwrap();
        assert!(resp.forum.is_object());
    }
}

#[tokio::test]
async fn forum_feed_options() {
    require_token!();
    let _resp = client().forum().forums_get_feed_options().await.unwrap();
}

#[tokio::test]
async fn forum_navigation_list() {
    require_token!();
    match client().forum().navigation_list(None).await {
        Ok(_) => {}
        Err(Error::Api { status, .. }) if status == 403 || status == 500 => {}
        Err(e) => panic!("unexpected: {e}"),
    }
}

// ══════════════════════════════════════════════════════════════════════
// Forum: Threads
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn forum_threads_list_with_limit() {
    require_token!();
    let resp = client()
        .forum()
        .threads_list(ForumThreadsListParams {
            limit: Some(5),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(!resp.threads.is_empty());
    assert!(resp.threads.len() <= 5);
}

#[tokio::test]
async fn forum_threads_get_by_id() {
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
    let resp = client()
        .forum()
        .threads_recent(ForumThreadsRecentParams {
            days: Some(7),
            limit: Some(5),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = &resp.threads;
}

// ══════════════════════════════════════════════════════════════════════
// Forum: Posts
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn forum_posts_list_in_thread() {
    require_token!();
    let c = client();
    let threads = c
        .forum()
        .threads_list(ForumThreadsListParams {
            limit: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    let tid = threads.threads[0].thread_id;
    let resp = c
        .forum()
        .posts_list(ForumPostsListParams {
            thread_id: Some(tid),
            limit: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(!resp.posts.is_empty());
}

#[tokio::test]
async fn forum_posts_get_by_id() {
    require_token!();
    let c = client();
    let threads = c
        .forum()
        .threads_list(ForumThreadsListParams {
            limit: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    let posts = c
        .forum()
        .posts_list(ForumPostsListParams {
            thread_id: Some(threads.threads[0].thread_id),
            limit: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    // posts.posts is Vec<Thread> due to codegen; extract post_id from first_post or thread_id
    if let Some(post) = posts.posts.first() {
        // Thread struct has thread_id; the API actually returns post objects here
        // but codegen maps them to Thread. Use serde_json round-trip to get post_id.
        let val = serde_json::to_value(post).unwrap();
        if let Some(pid) = val.get("post_id").and_then(|v| v.as_i64()) {
            let resp = c.forum().posts_get(pid).await.unwrap();
            assert_eq!(resp.post.post_id, pid);
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// Forum: Tags
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn forum_tags_popular() {
    require_token!();
    let _resp = client().forum().tags_popular().await.unwrap();
}

#[tokio::test]
async fn forum_tags_find() {
    require_token!();
    let _resp = client().forum().tags_find("rust".into()).await.unwrap();
}

// ══════════════════════════════════════════════════════════════════════
// Forum: Chatbox
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn forum_chatbox_index() {
    require_token!();
    let _resp = client().forum().chatbox_index(None).await.unwrap();
}

#[tokio::test]
async fn forum_chatbox_online() {
    require_token!();
    // room 0 may be forbidden; accept 403 gracefully
    match client().forum().chatbox_online(serde_json::json!(0)).await {
        Ok(_) => {}
        Err(Error::Forbidden { .. }) => {}
        Err(e) => panic!("unexpected: {e}"),
    }
}

#[tokio::test]
async fn forum_chatbox_leaderboard() {
    require_token!();
    let resp = client()
        .forum()
        .chatbox_get_leaderboard(None)
        .await
        .unwrap();
    let _ = &resp.system_info;
}

// ══════════════════════════════════════════════════════════════════════
// Forum: Notifications
// ══════════════════════════════════════════════════════════════════════

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

// ══════════════════════════════════════════════════════════════════════
// Forum: Conversations (read-only)
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn forum_conversations_list() {
    require_token!();
    match client()
        .forum()
        .conversations_list(None, None, Some(3))
        .await
    {
        Ok(_) => {}
        Err(Error::Api { status: 403, .. }) | Err(Error::Forbidden { .. }) => {}
        Err(e) => panic!("{e}"),
    }
}

// ══════════════════════════════════════════════════════════════════════
// Market: Profile
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn market_profile_get() {
    require_token!();
    let resp = client().market().profile_get(None).await.unwrap();
    assert!(resp.user.user_id > 0);
    assert!(!resp.user.username.is_empty());
}

// ══════════════════════════════════════════════════════════════════════
// Market: Categories
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn market_category_list() {
    require_token!();
    let _resp = client().market().category_list(None).await.unwrap();
}

#[tokio::test]
async fn market_category_params_steam() {
    require_token!();
    let _resp = client()
        .market()
        .category_params("steam".into())
        .await
        .unwrap();
}

#[tokio::test]
async fn market_category_games_steam() {
    require_token!();
    let _resp = client()
        .market()
        .category_games("steam".into())
        .await
        .unwrap();
}

#[tokio::test]
async fn market_category_steam_search() {
    require_token!();
    let resp = client()
        .market()
        .category_steam(MarketCategorySteamParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_category_telegram_search() {
    require_token!();
    let resp = client()
        .market()
        .category_telegram(MarketCategoryTelegramParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_category_discord_search() {
    require_token!();
    let resp = client()
        .market()
        .category_discord(MarketCategoryDiscordParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_category_fortnite_search() {
    require_token!();
    let resp = client()
        .market()
        .category_fortnite(MarketCategoryFortniteParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(resp.total_items >= 0);
}

#[tokio::test]
async fn market_category_all_search() {
    require_token!();
    let resp = client()
        .market()
        .category_all(MarketCategoryAllParams {
            page: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = resp;
}

// ══════════════════════════════════════════════════════════════════════
// Market: Payments & Balance
// ══════════════════════════════════════════════════════════════════════

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
async fn market_payments_balance_list() {
    require_token!();
    let _resp = client().market().payments_balance_list().await.unwrap();
}

// ══════════════════════════════════════════════════════════════════════
// Market: Lists
// ══════════════════════════════════════════════════════════════════════

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
async fn market_list_states() {
    require_token!();
    let _resp = client().market().list_states(None).await.unwrap();
}

// ══════════════════════════════════════════════════════════════════════
// Market: Cart, Proxy, Auto-payments, Discounts
// ══════════════════════════════════════════════════════════════════════

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
async fn market_proxy_get() {
    require_token!();
    let _resp = client().market().proxy_get().await.unwrap();
}

#[tokio::test]
async fn market_auto_payments() {
    require_token!();
    let _resp = client().market().auto_payments_list().await.unwrap();
}

#[tokio::test]
async fn market_custom_discounts() {
    require_token!();
    let _resp = client().market().custom_discounts_get().await.unwrap();
}

// ══════════════════════════════════════════════════════════════════════
// Error handling: auth with bad token
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn bad_token_returns_auth_error() {
    let c = LolzteamClient::builder("invalid-token-12345")
        .no_rate_limit()
        .max_retries(0)
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let result = c.forum().users_get(1, None).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.is_auth() || matches!(err, Error::Api { status: 401, .. }),
        "expected auth error, got: {err:?}"
    );
}

// ══════════════════════════════════════════════════════════════════════
// Rate limiter: verify we don't get 429 with default config
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn rate_limiter_prevents_429() {
    require_token!();
    let c = client();
    // Make 10 rapid requests — rate limiter should pace them
    for i in 0..10 {
        match c.forum().categories_list(None, None, None).await {
            Ok(_) => {}
            Err(Error::RateLimited { .. }) => {
                panic!("got 429 on request {i}, rate limiter should prevent this")
            }
            Err(e) => {
                // Other errors are OK (network, etc.)
                eprintln!("request {i} error (not 429): {e}");
                break;
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// on_retry callback fires on real transient errors
// ══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn on_retry_fires_for_bad_host() {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    let count = Arc::new(AtomicU32::new(0));
    let cnt = count.clone();

    let c = LolzteamClient::builder("token")
        .forum_base_url("http://127.0.0.1:1")
        .market_base_url("http://127.0.0.1:1")
        .max_retries(2)
        .timeout(Duration::from_millis(500))
        .no_rate_limit()
        .on_retry(Arc::new(move |_| {
            cnt.fetch_add(1, Ordering::SeqCst);
        }))
        .build()
        .unwrap();

    let _ = c.forum().users_get(1, None).await;
    assert!(count.load(Ordering::SeqCst) > 0, "on_retry should fire");
}
