// cargo run --example live_full_test -- YOUR_TOKEN
//
// Comprehensive live test for ALL safe (read-only GET) endpoints.
// Does NOT call any POST/PUT/DELETE that could modify profile, spend money,
// or leave comments.

use lolzteam::forum::types::*;
use lolzteam::market::types::*;
use lolzteam::LolzteamClient;
use std::time::Duration;

macro_rules! test_call {
    ($label:expr, $call:expr) => {{
        print!("{} ... ", $label);
        match $call.await {
            Ok(resp) => {
                let json = serde_json::to_string(&resp).unwrap_or_default();
                let preview = if json.len() > 200 {
                    let mut end = 200;
                    while !json.is_char_boundary(end) && end > 0 {
                        end -= 1;
                    }
                    format!("{}…", &json[..end])
                } else {
                    json.clone()
                };
                println!("✅  ({} bytes) {}", json.len(), preview);
                tokio::time::sleep(Duration::from_millis(500)).await;
                Some(resp)
            }
            Err(e) => {
                println!("❌  {e}");
                tokio::time::sleep(Duration::from_millis(300)).await;
                None
            }
        }
    }};
}

/// Like test_call! but treats certain expected errors (403/404) as a pass (⚠️).
macro_rules! test_call_tolerant {
    ($label:expr, $call:expr) => {{
        print!("{} ... ", $label);
        match $call.await {
            Ok(resp) => {
                let json = serde_json::to_string(&resp).unwrap_or_default();
                let preview = if json.len() > 200 {
                    let mut end = 200;
                    while !json.is_char_boundary(end) && end > 0 {
                        end -= 1;
                    }
                    format!("{}…", &json[..end])
                } else {
                    json.clone()
                };
                println!("✅  ({} bytes) {}", json.len(), preview);
                tokio::time::sleep(Duration::from_millis(500)).await;
                (true, true) // (counted_ok, is_real_success)
            }
            Err(ref e) => {
                let msg = format!("{e}");
                let expected = msg.contains("403")
                    || msg.contains("404")
                    || msg.contains("Forbidden")
                    || msg.contains("Not found")
                    || msg.contains("не найден")
                    || msg.contains("нет прав")
                    || msg.contains("не можете")
                    || msg.contains("не существует");
                if expected {
                    println!("⚠️  (expected) {e}");
                } else {
                    println!("❌  {e}");
                }
                tokio::time::sleep(Duration::from_millis(300)).await;
                (expected, false)
            }
        }
    }};
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: live_full_test <TOKEN>");
    let client = LolzteamClient::builder(&token)
        .timeout(Duration::from_secs(60))
        .build()
        .expect("failed to build client");
    let forum = client.forum();
    let market = client.market();

    let mut ok = 0u32;
    let mut fail = 0u32;
    let mut warned = 0u32;
    let mut total = 0u32;

    macro_rules! count {
        ($label:expr, $call:expr) => {{
            total += 1;
            let res = test_call!($label, $call);
            if res.is_some() {
                ok += 1;
            } else {
                fail += 1;
            }
            res
        }};
    }

    /// Count but treat expected 403/404 as a warning, not failure.
    macro_rules! count_tolerant {
        ($label:expr, $call:expr) => {{
            total += 1;
            let (counted_ok, _real_success) = test_call_tolerant!($label, $call);
            if counted_ok {
                ok += 1;
                if !_real_success {
                    warned += 1;
                }
            } else {
                fail += 1;
            }
        }};
    }

    println!("╔══════════════════════════════════════════════════════╗");
    println!("║        LOLZTEAM API — COMPREHENSIVE LIVE TEST       ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════
    // FORUM API — Read-only endpoints
    // ═══════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  FORUM API");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // ── Assets ──
    println!("── Assets ──");
    count!("forum.assets_css", forum.assets_css(None));

    // ── Categories ──
    println!("\n── Categories ──");
    let cats = count!(
        "forum.categories_list",
        forum.categories_list(None, None, None)
    );
    // Pick a real category_id from the list (first one), fallback to 103
    let first_cat_id = cats
        .as_ref()
        .and_then(|r| r.categories.first().map(|c| c.category_id))
        .unwrap_or(103);
    count!(
        &format!("forum.categories_get({})", first_cat_id),
        forum.categories_get(first_cat_id)
    );

    // ── Forums ──
    println!("\n── Forums ──");
    let forums_resp = count!("forum.forums_list", forum.forums_list(None, None, None));
    let first_forum_id = forums_resp
        .as_ref()
        .and_then(|r| r.forums.first().map(|f| f.forum_id))
        .unwrap_or(2);
    count!(
        &format!("forum.forums_get({})", first_forum_id),
        forum.forums_get(first_forum_id)
    );
    count!("forum.forums_grouped", forum.forums_grouped());
    count!(
        "forum.forums_get_feed_options",
        forum.forums_get_feed_options()
    );
    count!("forum.forums_followed", forum.forums_followed(None));
    count!(
        &format!("forum.forums_followers({})", first_forum_id),
        forum.forums_followers(first_forum_id)
    );

    // ── Link Forums ──
    println!("\n── Link Forums ──");
    count!("forum.links_list", forum.links_list());

    // ── Navigation ──
    println!("\n── Navigation ──");
    count!("forum.navigation_list", forum.navigation_list(None));

    // ── Pages ──
    println!("\n── Pages ──");
    count!("forum.pages_list", forum.pages_list(None, None));

    // ── Notifications ──
    println!("\n── Notifications ──");
    count!(
        "forum.notifications_list",
        forum.notifications_list(None, None, None)
    );

    // ── Tags ──
    println!("\n── Tags ──");
    count!("forum.tags_popular", forum.tags_popular());
    count!("forum.tags_list", forum.tags_list(None, None));
    count!("forum.tags_find('rust')", forum.tags_find("rust".into()));

    // ── Chatbox ──
    println!("\n── Chatbox ──");
    count!("forum.chatbox_index", forum.chatbox_index(None));
    count!(
        "forum.chatbox_get_messages",
        forum.chatbox_get_messages(serde_json::Value::Null, None)
    );
    count!("forum.chatbox_get_ignore", forum.chatbox_get_ignore());
    count!(
        "forum.chatbox_get_leaderboard",
        forum.chatbox_get_leaderboard(None)
    );
    // chatbox_online may require specific room permissions
    count_tolerant!(
        "forum.chatbox_online",
        forum.chatbox_online(serde_json::Value::Null)
    );

    // ── Threads ──
    println!("\n── Threads ──");
    count!(
        "forum.threads_list(forum_id, page=1)",
        forum.threads_list(ForumThreadsListParams {
            forum_id: Some(first_forum_id),
            page: Some(1),
            limit: Some(5),
            ..Default::default()
        })
    );
    count!(
        "forum.threads_recent(limit=5)",
        forum.threads_recent(ForumThreadsRecentParams {
            limit: Some(5),
            ..Default::default()
        })
    );
    // threads_unread params: (limit, forum_id, data_limit) — use first_forum_id
    count!(
        &format!("forum.threads_unread(forum_id={})", first_forum_id),
        forum.threads_unread(Some(5), Some(first_forum_id), None)
    );
    count!(
        "forum.threads_followed(page=1)",
        forum.threads_followed(None, None)
    );

    // Get a real thread_id
    let thread_id = {
        let resp = forum
            .threads_list(ForumThreadsListParams {
                forum_id: Some(first_forum_id),
                limit: Some(1),
                ..Default::default()
            })
            .await;
        resp.ok()
            .and_then(|r| r.threads.first().map(|t| t.thread_id))
            .unwrap_or(1)
    };

    count!(
        &format!("forum.threads_get({})", thread_id),
        forum.threads_get(thread_id, None)
    );
    count!(
        &format!("forum.threads_navigation({})", thread_id),
        forum.threads_navigation(thread_id)
    );
    // Not all threads have polls — expected 404 is acceptable
    count_tolerant!(
        &format!("forum.threads_poll_get({})", thread_id),
        forum.threads_poll_get(thread_id)
    );
    count!(
        &format!("forum.threads_followers({})", thread_id),
        forum.threads_followers(thread_id)
    );

    // ── Posts ──
    println!("\n── Posts ──");
    count!(
        "forum.posts_list(thread_id, limit=5)",
        forum.posts_list(ForumPostsListParams {
            thread_id: Some(thread_id),
            limit: Some(5),
            ..Default::default()
        })
    );

    // Get a real post_id
    let post_id = {
        let resp = forum
            .posts_list(ForumPostsListParams {
                thread_id: Some(thread_id),
                limit: Some(1),
                ..Default::default()
            })
            .await;
        resp.ok()
            .and_then(|r| r.posts.first().map(|p| p.post_id))
            .unwrap_or(1)
    };

    count!(
        &format!("forum.posts_get({})", post_id),
        forum.posts_get(post_id)
    );
    count!(
        &format!("forum.posts_likes({})", post_id),
        forum.posts_likes(post_id, None, None)
    );
    // Can't always report — expected 403
    count_tolerant!(
        &format!("forum.posts_report_reasons({})", post_id),
        forum.posts_report_reasons(post_id)
    );
    count!(
        &format!("forum.posts_comments_get({})", post_id),
        forum.posts_comments_get(post_id, None, None)
    );

    // ── Users ──
    println!("\n── Users ──");
    count!("forum.users_get(1)", forum.users_get(1, None));

    // Get own user_id from users_get
    let own_user = forum.users_get(5285311, None).await;
    let own_id = own_user.ok().map(|u| u.user.user_id).unwrap_or(5285311);

    count!(
        &format!("forum.users_get(self={})", own_id),
        forum.users_get(own_id, None)
    );
    count!(
        "forum.users_find('AS7RIDENIED')",
        forum.users_find(Some("AS7RIDENIED".into()), None, None)
    );
    count!("forum.users_fields", forum.users_fields());
    count!(
        "forum.users_followers(1)",
        forum.users_followers(1, None, None, None)
    );
    count!(
        "forum.users_followings(1)",
        forum.users_followings(1, None, None, None)
    );
    count!("forum.users_trophies(1)", forum.users_trophies(1));
    count!(
        "forum.users_contents(1) timeline",
        forum.users_contents(1, None, None)
    );
    count!("forum.users_ignored", forum.users_ignored(None));
    count!(
        "forum.users_secret_answer_types",
        forum.users_secret_answer_types()
    );
    count!(
        &format!("forum.users_likes({})", own_id),
        forum.users_likes(own_id, ForumUsersLikesParams::default())
    );
    count!(
        &format!("forum.users_claims({})", own_id),
        forum.users_claims(own_id, None, None)
    );

    // ── Profile Posts ──
    println!("\n── Profile Posts ──");
    count!(
        &format!("forum.profile_posts_list(user_id={})", 1),
        forum.profile_posts_list(1, ForumProfilePostsListParams::default())
    );

    // ── Conversations ──
    println!("\n── Conversations ──");
    // conversations may require extra permissions
    count_tolerant!(
        "forum.conversations_list",
        forum.conversations_list(None, None, None)
    );

    // ── Forms ──
    println!("\n── Forms ──");
    count!("forum.forms_list", forum.forms_list(None));

    // ═══════════════════════════════════════════════════════════════
    // MARKET API — Read-only endpoints
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  MARKET API");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // ── Profile ──
    println!("── Profile ──");
    count!("market.profile_get", market.profile_get(None));

    // ── Categories ──
    println!("\n── Categories ──");
    count!("market.category_list", market.category_list(None));
    count!(
        "market.category_list(top_queries=true)",
        market.category_list(Some(true))
    );
    count!(
        "market.category_params('steam')",
        market.category_params("steam".into())
    );
    count!(
        "market.category_games('steam')",
        market.category_games("steam".into())
    );

    // ── Category Search (all platforms) ──
    println!("\n── Category Search ──");
    count!(
        "market.category_all(page=1)",
        market.category_all(MarketCategoryAllParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_steam(page=1)",
        market.category_steam(MarketCategorySteamParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_fortnite(page=1)",
        market.category_fortnite(MarketCategoryFortniteParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_telegram(page=1)",
        market.category_telegram(MarketCategoryTelegramParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_discord(page=1)",
        market.category_discord(MarketCategoryDiscordParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_riot(page=1)",
        market.category_riot(MarketCategoryRiotParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_minecraft(page=1)",
        market.category_minecraft(MarketCategoryMinecraftParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_roblox(page=1)",
        market.category_roblox(MarketCategoryRobloxParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_ea(page=1)",
        market.category_ea(MarketCategoryEaParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_epic_games(page=1)",
        market.category_epic_games(MarketCategoryEpicGamesParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_escape_from_tarkov(page=1)",
        market.category_escape_from_tarkov(MarketCategoryEscapeFromTarkovParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_social_club(page=1)",
        market.category_social_club(MarketCategorySocialClubParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_uplay(page=1)",
        market.category_uplay(MarketCategoryUplayParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_wot(page=1)",
        market.category_wot(MarketCategoryWotParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_wot_blitz(page=1)",
        market.category_wot_blitz(MarketCategoryWotBlitzParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_warface(page=1)",
        market.category_warface(MarketCategoryWarfaceParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_supercell(page=1)",
        market.category_supercell(MarketCategorySupercellParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_mihoyo(page=1)",
        market.category_mihoyo(MarketCategoryMihoyoParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_battle_net(page=1)",
        market.category_battle_net(MarketCategoryBattleNetParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_chat_gpt(page=1)",
        market.category_chat_gpt(MarketCategoryChatGptParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_instagram(page=1)",
        market.category_instagram(MarketCategoryInstagramParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_tik_tok(page=1)",
        market.category_tik_tok(MarketCategoryTikTokParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_vpn(page=1)",
        market.category_vpn(MarketCategoryVpnParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_gifts(page=1)",
        market.category_gifts(MarketCategoryGiftsParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.category_hytale(page=1)",
        market.category_hytale(MarketCategoryHytaleParams {
            page: Some(1),
            ..Default::default()
        })
    );

    // ── Accounts list ──
    println!("\n── Accounts list ──");
    count!(
        "market.list_user(page=1)",
        market.list_user(MarketListUserParams {
            page: Some(1),
            ..Default::default()
        })
    );
    count!(
        "market.list_favorites",
        market.list_favorites(MarketListFavoritesParams::default())
    );
    count!(
        "market.list_viewed",
        market.list_viewed(MarketListViewedParams::default())
    );
    count!(
        "market.list_orders",
        market.list_orders(MarketListOrdersParams::default())
    );
    count!("market.list_states", market.list_states(None));

    // ── Payments ──
    println!("\n── Payments ──");
    count!(
        "market.payments_history",
        market.payments_history(MarketPaymentsHistoryParams::default())
    );
    count!("market.payments_currency", market.payments_currency());
    count!("market.payments_fee", market.payments_fee(Some(100.0)));
    count!(
        "market.payments_balance_list",
        market.payments_balance_list()
    );
    count!(
        "market.payments_payout_services",
        market.payments_payout_services()
    );
    count!("market.auto_payments_list", market.auto_payments_list());

    // ── Invoices ──
    println!("\n── Invoices ──");
    count!(
        "market.payments_invoice_list",
        market.payments_invoice_list(MarketPaymentsInvoiceListParams::default())
    );

    // ── Steam value ──
    println!("\n── Steam value ──");
    // managing_steam_value requires a link, skip if no link available

    // ── Cart ──
    println!("\n── Cart ──");
    count!(
        "market.cart_get",
        market.cart_get(MarketCartGetParams::default())
    );

    // ── Custom discounts ──
    println!("\n── Custom discounts ──");
    count!("market.custom_discounts_get", market.custom_discounts_get());

    // ── Proxy ──
    println!("\n── Proxy ──");
    count!("market.proxy_get", market.proxy_get());

    // ── Profile claims ──
    println!("\n── Profile claims ──");
    count!("market.profile_claims", market.profile_claims(None, None));

    // ── Letters2 ──
    println!("\n── Letters2 ──");
    // managing_get_letters2 requires email credentials, skip

    // ── Get specific item details ──
    println!("\n── Item details ──");

    // First try to get our own items (list_user shows own items)
    let own_items = market
        .list_user(MarketListUserParams {
            page: Some(1),
            ..Default::default()
        })
        .await;
    let own_item_id = own_items
        .ok()
        .and_then(|r| r.items.first().and_then(|v| v.item_id));

    // Get a steam item for general viewing
    let steam_items = market
        .category_steam(MarketCategorySteamParams {
            page: Some(1),
            ..Default::default()
        })
        .await;
    let steam_item_id = steam_items.as_ref().ok().and_then(|r| {
        r.items
            .first()
            .and_then(|v| v.get("item_id").and_then(|id| id.as_i64()))
    });

    if let Some(item_id) = steam_item_id {
        count!(
            &format!("market.managing_get({})", item_id),
            market.managing_get(item_id, None)
        );
        // managing_image type should be "games" not "main"
        count_tolerant!(
            &format!("market.managing_image({}, games)", item_id),
            market.managing_image(item_id, "games".to_string())
        );
        // steam_preview may not always be available
        count_tolerant!(
            &format!("market.managing_steam_preview({})", item_id),
            market.managing_steam_preview(item_id, None)
        );
        count_tolerant!(
            &format!("market.managing_steam_inventory_value({})", item_id),
            market.managing_steam_inventory_value(item_id, None, None, None)
        );
    }

    // These require item ownership — use own item if available, otherwise use
    // a random steam item and tolerate 403.
    if let Some(item_id) = own_item_id {
        count_tolerant!(
            &format!("market.managing_ai_price(own:{})", item_id),
            market.managing_ai_price(item_id)
        );
        count_tolerant!(
            &format!("market.managing_auto_buy_price(own:{})", item_id),
            market.managing_auto_buy_price(item_id)
        );
    } else if let Some(item_id) = steam_item_id {
        // Not our item — 403 expected
        count_tolerant!(
            &format!("market.managing_ai_price({})", item_id),
            market.managing_ai_price(item_id)
        );
        count_tolerant!(
            &format!("market.managing_auto_buy_price({})", item_id),
            market.managing_auto_buy_price(item_id)
        );
    }

    // ═══════════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════════
    println!("\n╔══════════════════════════════════════════════════════╗");
    println!("║                    TEST SUMMARY                     ║");
    println!("╠══════════════════════════════════════════════════════╣");
    println!(
        "║  Total : {:<5}                                     ║",
        total
    );
    println!(
        "║  ✅ OK  : {:<5}  (of which ⚠️  expected: {:<5})     ║",
        ok, warned
    );
    println!(
        "║  ❌ Fail: {:<5}                                     ║",
        fail
    );
    println!(
        "║  Rate  : {:.1}%                                      ║",
        if total > 0 {
            ok as f64 / total as f64 * 100.0
        } else {
            0.0
        }
    );
    println!("╚══════════════════════════════════════════════════════╝");

    if fail > 0 {
        std::process::exit(1);
    }
    Ok(())
}
