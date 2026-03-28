# Documentation — lolzteam

Full method reference for the LOLZTEAM Forum and Market API Rust client.

## Client Configuration

| Method | Description | Default |
|---|---|---|
| `LolzteamClient::new(token)` | Create client with defaults | — |
| `.builder(token)` | Create builder for fine-tuning | — |
| `.proxy(url)` | Shared proxy for Forum and Market | None |
| `.forum_proxy(url)` | Proxy for Forum API only | None |
| `.market_proxy(url)` | Proxy for Market API only | None |
| `.max_retries(n)` | Max retry attempts | 5 |
| `.timeout(duration)` | Request timeout | 30 sec |
| `.forum_base_url(url)` | Custom Forum API URL | `https://prod-api.lolz.live` |
| `.market_base_url(url)` | Custom Market API URL | `https://prod-api.lzt.market` |

## Error Types

| Type | Description |
|---|---|
| `Error::Http` | Network error (timeout, DNS, connection) |
| `Error::Json` | JSON parsing error |
| `Error::Auth` | 401 Unauthorized |
| `Error::Forbidden` | 403 Forbidden |
| `Error::NotFound` | 404 Not Found |
| `Error::RateLimited` | All retry attempts exhausted |
| `Error::Api` | Other HTTP errors from the API |

## Code Generation

Generated files:
- `src/models.rs` — response models (shared between Forum and Market)
- `src/forum/types.rs` — Forum API request parameter types
- `src/forum/methods.rs` — Forum API methods
- `src/market/types.rs` — Market API request parameter types
- `src/market/methods.rs` — Market API methods

Regenerate:
```bash
make generate
```

---

## Forum API

### Categories
| Method | Description |
|---|---|
| `categories_get(category_id)` | Get category |
| `categories_list(parent_category_id, parent_forum_id, order)` | List categories |

### Forums
| Method | Description |
|---|---|
| `forums_get(forum_id)` | Get forum |
| `forums_list(parent_category_id, parent_forum_id, order)` | List forums |
| `forums_grouped()` | Forums grouped by category |
| `forums_follow(params)` | Follow a forum |
| `forums_unfollow(forum_id)` | Unfollow a forum |
| `forums_followed(total)` | List followed forums |
| `forums_followers(forum_id)` | Forum followers |
| `forums_get_feed_options()` | Get feed options |
| `forums_edit_feed_options(...)` | Edit feed options |

### Threads
| Method | Description |
|---|---|
| `threads_list(params)` | List threads |
| `threads_get(thread_id, ...)` | Get thread |
| `threads_create(params)` | Create thread |
| `threads_edit(params)` | Edit thread |
| `threads_delete(thread_id, ...)` | Delete thread |
| `threads_move(thread_id, params)` | Move thread |
| `threads_bump(thread_id)` | Bump thread |
| `threads_recent(params)` | Recent threads |
| `threads_unread(...)` | Unread threads |
| `threads_create_contest(params)` | Create contest |
| `threads_claim(params)` | Create claim |
| `threads_follow(thread_id, ...)` | Follow thread |
| `threads_unfollow(thread_id)` | Unfollow thread |
| `threads_followed(...)` | Followed threads |
| `threads_followers(thread_id)` | Thread followers |
| `threads_poll_get(thread_id)` | Get poll |
| `threads_poll_vote(thread_id, ...)` | Vote in poll |
| `threads_star(thread_id)` | Star thread |
| `threads_unstar(thread_id)` | Unstar thread |
| `threads_hide(thread_id)` | Hide thread |
| `threads_finish(thread_id)` | Finish thread |
| `threads_navigation(thread_id)` | Thread navigation |

### Posts
| Method | Description |
|---|---|
| `posts_list(params)` | List posts |
| `posts_get(post_id)` | Get post |
| `posts_create(thread_id, post_body)` | Create post |
| `posts_edit(post_id, ...)` | Edit post |
| `posts_delete(post_id, ...)` | Delete post |
| `posts_like(post_id)` | Like post |
| `posts_unlike(post_id)` | Unlike post |
| `posts_likes(post_id, ...)` | List likes |
| `posts_report(post_id, message)` | Report post |
| `posts_report_reasons(post_id)` | Report reasons |
| `posts_comments_create(...)` | Create post comment |
| `posts_comments_get(...)` | Get post comment |
| `posts_comments_edit(...)` | Edit post comment |
| `posts_comments_delete(...)` | Delete post comment |

### Profile Posts
| Method | Description |
|---|---|
| `profile_posts_list(params)` | List profile posts |
| `profile_posts_get(profile_post_id)` | Get profile post |
| `profile_posts_create(...)` | Create profile post |
| `profile_posts_edit(...)` | Edit profile post |
| `profile_posts_delete(...)` | Delete profile post |
| `profile_posts_like(profile_post_id)` | Like |
| `profile_posts_unlike(profile_post_id)` | Unlike |
| `profile_posts_stick(profile_post_id)` | Pin |
| `profile_posts_unstick(profile_post_id)` | Unpin |
| `profile_posts_comments_list(...)` | List profile post comments |
| `profile_posts_comments_create(...)` | Create comment |
| `profile_posts_comments_edit(...)` | Edit comment |
| `profile_posts_comments_delete(...)` | Delete comment |

### Users
| Method | Description |
|---|---|
| `users_get(user_id, ...)` | Get user |
| `users_find(username, ...)` | Find user |
| `users_list(page, limit, ...)` | List users |
| `users_edit(user_id, params)` | Edit profile |
| `users_follow(user_id)` | Follow |
| `users_unfollow(user_id)` | Unfollow |
| `users_followers(user_id, ...)` | Followers |
| `users_followings(user_id, ...)` | Followings |
| `users_ignore(user_id)` | Ignore |
| `users_unignore(user_id)` | Unignore |
| `users_ignored(total)` | Ignored list |
| `users_likes(user_id, params)` | User likes |
| `users_trophies(user_id)` | Trophies |
| `users_contents(user_id, ...)` | User content |
| `users_claims(user_id, ...)` | User claims |
| `users_fields()` | Profile fields |
| `users_avatar_upload(user_id, params)` | Upload avatar |
| `users_avatar_delete(user_id)` | Delete avatar |
| `users_background_upload(user_id, params)` | Upload background |
| `users_background_delete(user_id)` | Delete background |

### Conversations
| Method | Description |
|---|---|
| `conversations_list(page, limit, ...)` | List conversations |
| `conversations_get(conversation_id)` | Get conversation |
| `conversations_create(params)` | Create conversation |
| `conversations_update(params)` | Update conversation |
| `conversations_delete(conversation_id)` | Delete conversation |
| `conversations_read(conversation_id)` | Mark as read |
| `conversations_read_all()` | Mark all as read |
| `conversations_messages_list(conversation_id, params)` | List messages |
| `conversations_messages_get(...)` | Get message |
| `conversations_messages_create(...)` | Send message |
| `conversations_messages_edit(...)` | Edit message |
| `conversations_messages_delete(...)` | Delete message |
| `conversations_search(...)` | Search conversations |
| `conversations_star(conversation_id)` | Star |
| `conversations_unstar(conversation_id)` | Unstar |
| `conversations_invite(...)` | Invite to conversation |
| `conversations_kick(...)` | Kick from conversation |

### Search
| Method | Description |
|---|---|
| `search_all(params)` | Search everything |
| `search_threads(params)` | Search threads |
| `search_posts(params)` | Search posts |
| `search_profile_posts(params)` | Search profile posts |
| `search_tagged(params)` | Search by tags |
| `search_users(q)` | Search users |
| `search_results(search_id, ...)` | Search results |

### Notifications
| Method | Description |
|---|---|
| `notifications_list(page, limit, ...)` | List notifications |
| `notifications_get(notification_id)` | Get notification |
| `notifications_read(notification_id)` | Mark as read |

### Tags
| Method | Description |
|---|---|
| `tags_popular()` | Popular tags |
| `tags_list(page, limit)` | List tags |
| `tags_get(tag_id)` | Get tag |
| `tags_find(tag)` | Find tag |

### Chatbox
| Method | Description |
|---|---|
| `chatbox_index(...)` | Chatbox index |
| `chatbox_get_messages(...)` | Get messages |
| `chatbox_post_message(...)` | Post message |
| `chatbox_edit_message(...)` | Edit message |
| `chatbox_delete_message(message_id)` | Delete message |
| `chatbox_online(...)` | Online users |
| `chatbox_get_leaderboard(...)` | Leaderboard |
| `chatbox_report(...)` | Report |
| `chatbox_report_reasons()` | Report reasons |
| `chatbox_get_ignore()` | Ignore list |
| `chatbox_post_ignore(...)` | Add to ignore |
| `chatbox_delete_ignore(...)` | Remove from ignore |

### Other
| Method | Description |
|---|---|
| `navigation_list(parent)` | Navigation |
| `o_auth_token()` | Get access token |
| `batch_execute()` | Batch request |
| `assets_css(css)` | Get CSS |
| `forms_list(page)` | List forms |
| `forms_create()` | Create form |
| `pages_list(...)` | List pages |
| `pages_get(page_id)` | Get page |
| `links_list()` | List links |
| `links_get(link_id)` | Get link |

---

## Market API

### Categories
| Method | Description |
|---|---|
| `category_list(top_queries)` | List categories |
| `category_params(category_name)` | Category parameters |
| `category_games(category_name)` | Category games |

### Category Search
| Method | Description |
|---|---|
| `category_all(params)` | Search all categories |
| `category_steam(params)` | Steam accounts |
| `category_fortnite(params)` | Fortnite accounts |
| `category_discord(params)` | Discord accounts |
| `category_telegram(params)` | Telegram accounts |
| `category_minecraft(params)` | Minecraft accounts |
| `category_roblox(params)` | Roblox accounts |
| `category_riot(params)` | Riot Games accounts |
| `category_epic_games(params)` | Epic Games accounts |
| `category_ea(params)` | EA accounts |
| `category_battle_net(params)` | Battle.net accounts |
| `category_uplay(params)` | Uplay accounts |
| `category_social_club(params)` | Social Club accounts |
| `category_wot(params)` | World of Tanks accounts |
| `category_wot_blitz(params)` | WoT Blitz accounts |
| `category_warface(params)` | Warface accounts |
| `category_supercell(params)` | Supercell accounts |
| `category_mihoyo(params)` | miHoYo accounts |
| `category_escape_from_tarkov(params)` | Escape from Tarkov accounts |
| `category_instagram(params)` | Instagram accounts |
| `category_tik_tok(params)` | TikTok accounts |
| `category_chat_gpt(params)` | ChatGPT accounts |
| `category_vpn(params)` | VPN accounts |
| `category_gifts(params)` | Gift cards |
| `category_hytale(params)` | Hytale accounts |

### Account Lists
| Method | Description |
|---|---|
| `list_user(params)` | My accounts |
| `list_favorites(params)` | Favorites |
| `list_orders(params)` | Orders |
| `list_viewed(params)` | Viewed |
| `list_states(user_id)` | Account states |
| `list_download(params)` | Download list |

### Account Managing
| Method | Description |
|---|---|
| `managing_get(item_id)` | Get account |
| `managing_edit(params)` | Edit account |
| `manging_delete(item_id, reason)` | Delete account |
| `managing_bump(item_id)` | Bump account |
| `managing_auto_bump(item_id, hour)` | Auto-bump |
| `managing_auto_bump_disable(item_id)` | Disable auto-bump |
| `managing_auto_buy_price(params)` | Auto-buy price |
| `managing_stick(item_id)` | Stick |
| `managing_unstick(item_id)` | Unstick |
| `managing_favorite(item_id)` | Add to favorites |
| `managing_unfavorite(item_id)` | Remove from favorites |
| `managing_tag(item_id, tag_id)` | Add tag |
| `managing_untag(item_id, tag_id)` | Remove tag |
| `managing_public_tag(params)` | Public tag |
| `managing_public_untag(params)` | Remove public tag |
| `managing_note(params)` | Note |
| `managing_image(params)` | Image |
| `managing_close(item_id)` | Close |
| `managing_open(item_id)` | Open |
| `managing_transfer(params)` | Transfer account |
| `managing_change_password(params)` | Change password |
| `managing_ai_price(item_id)` | AI price estimation |
| `managing_bulk_get(params)` | Bulk get |
| `managing_create_claim(params)` | Create claim |
| `managing_check_guarantee(item_id)` | Check guarantee |
| `managing_refuse_guarantee(item_id)` | Refuse guarantee |
| `managing_email_code(item_id)` | Email code |
| `managing_telegram_code(params)` | Telegram code |
| `managing_temp_email_password(params)` | Temp email password |
| `managing_steam_value(params)` | Steam value |
| `managing_steam_update_value(params)` | Update Steam value |
| `managing_steam_inventory_value(params)` | Inventory value |
| `managing_steam_preview(params)` | Steam preview |
| `managing_steam_mafile_code(item_id)` | maFile code |
| `managing_steam_get_mafile(params)` | Get maFile |
| `managing_steam_add_mafile(item_id)` | Add maFile |
| `managing_steam_remove_mafile(item_id)` | Remove maFile |
| `managing_steam_sda(params)` | Steam Desktop Auth |

### Account Publishing
| Method | Description |
|---|---|
| `publishing_add(params)` | Add account |
| `publishing_check(params)` | Check account |
| `publishing_fast_sell(params)` | Fast sell |
| `publishing_external(params)` | External account |

### Account Purchasing
| Method | Description |
|---|---|
| `purchasing_fast_buy(params)` | Fast buy |
| `purchasing_confirm(params)` | Confirm purchase |
| `purchasing_check(item_id)` | Check purchase |
| `purchasing_discount_request(params)` | Request discount |
| `purchasing_discount_cancel(item_id)` | Cancel discount request |

### Cart
| Method | Description |
|---|---|
| `cart_get(params)` | Get cart |
| `cart_add(item_id)` | Add to cart |
| `cart_delete(params)` | Remove from cart |

### Payments
| Method | Description |
|---|---|
| `payments_history(params)` | Payment history |
| `payments_currency()` | Currency rates |
| `payments_fee(amount)` | Fee |
| `payments_transfer(params)` | Transfer |
| `payments_payout(params)` | Payout |
| `payments_payout_services()` | Payout services |
| `payments_cancel(payment_id)` | Cancel payment |
| `payments_balance_list()` | Balance list |
| `payments_balance_exchange(amount, from, to)` | Currency exchange |

### Invoices
| Method | Description |
|---|---|
| `payments_invoice_create(params)` | Create invoice |
| `payments_invoice_get(invoice_id, payment_id)` | Get invoice |
| `payments_invoice_list(params)` | List invoices |

### Auto Payments
| Method | Description |
|---|---|
| `auto_payments_list()` | List auto payments |
| `auto_payments_create(params)` | Create auto payment |
| `auto_payments_delete(auto_payment_id)` | Delete auto payment |

### Custom Discounts
| Method | Description |
|---|---|
| `custom_discounts_get()` | Get discounts |
| `custom_discounts_create(params)` | Create discount |
| `custom_discounts_edit(params)` | Edit discount |
| `custom_discounts_delete(discount_id)` | Delete discount |

### Profile
| Method | Description |
|---|---|
| `profile_get(fields_include)` | Get profile |
| `profile_edit(params)` | Edit profile |
| `profile_claims(params)` | Profile claims |

### Proxy
| Method | Description |
|---|---|
| `proxy_get()` | Get proxies |
| `proxy_add(params)` | Add proxy |
| `proxy_delete(delete_all, proxy_id)` | Delete proxy |

### IMAP
| Method | Description |
|---|---|
| `imap_create(params)` | Create IMAP |
| `imap_delete(domain)` | Delete IMAP |

### Other
| Method | Description |
|---|---|
| `batch()` | Batch request |


