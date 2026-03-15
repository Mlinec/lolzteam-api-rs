//! Auto-generated response models from the LOLZTEAM OpenAPI specification.
//!
//! DO NOT EDIT — regenerate with `cargo run -p lolzteam-codegen`.

#![allow(unused, clippy::all)]

use serde::{Deserialize, Serialize};

/// ChatboxMessage model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxMessage {
    pub can_report: bool,
    pub date: i64,
    pub is_deleted: bool,
    pub message: String,
    #[serde(rename = "messageJson")]
    pub message_json: String,
    #[serde(rename = "messageRaw")]
    pub message_raw: String,
    pub message_id: i64,
    pub room: serde_json::Value,
    pub user: serde_json::Value,
}

/// ConversationMessage model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationMessage {
    pub conversation_id: i64,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub links: serde_json::Value,
    pub message_body: String,
    pub message_body_html: String,
    pub message_body_plain_text: String,
    pub message_create_date: i64,
    pub message_edit_date: i64,
    pub message_id: i64,
    pub message_is_system: bool,
    pub message_is_unread: i64,
    pub message_need_translate: bool,
    pub permissions: serde_json::Value,
    pub user_is_ignored: bool,
}

/// Conversation model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Conversation {
    pub alerts: i64,
    pub conversation_create_date: i64,
    pub conversation_id: i64,
    pub conversation_is_deleted: bool,
    pub conversation_is_new: bool,
    pub conversation_is_open: bool,
    pub conversation_last_read_date: i64,
    pub conversation_message_count: i64,
    pub conversation_online_count: i64,
    pub conversation_title: String,
    pub conversation_update_date: i64,
    pub creator_is_ignored: bool,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub is_group: i64,
    pub is_starred: i64,
    pub is_unread: i64,
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub recipient: serde_json::Value,
    pub recipients: Vec<serde_json::Value>,
}

/// Link model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Link {
    pub link_description: String,
    pub link_id: i64,
    pub link_title: String,
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
}

/// Notification model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Notification {
    pub content_action: String,
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub links: serde_json::Value,
    pub notification_create_date: i64,
    pub notification_html: String,
    pub notification_id: i64,
    pub notification_is_unread: bool,
    pub notification_type: String,
}

/// PostComment model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostComment {
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub post_comment_body: String,
    pub post_comment_body_html: String,
    pub post_comment_body_plain_text: String,
    pub post_comment_create_date: i64,
    pub post_comment_id: i64,
    pub post_comment_is_deleted: bool,
    pub post_comment_is_published: bool,
    pub post_comment_like_count: i64,
    pub post_comment_update_date: i64,
    pub post_id: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}

/// Post model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Post {
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub user_is_ignored: bool,
}

/// ProfilePostComment model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostComment {
    pub comment_body: String,
    pub comment_body_html: String,
    pub comment_body_plain_text: String,
    pub comment_create_date: i64,
    pub comment_id: i64,
    pub comment_user_id: i64,
    pub comment_username: String,
    pub comment_username_html: String,
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub profile_post_id: i64,
    pub timeline_user_id: i64,
    pub user_is_ignored: bool,
}

/// ProfilePost model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePost {
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_comment_count: i64,
    pub post_comments_is_disabled: i64,
    pub post_create_date: i64,
    pub post_is_deleted: bool,
    pub post_is_liked: bool,
    pub post_is_published: bool,
    pub post_is_sticked: bool,
    pub post_like_count: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub profile_post_id: i64,
    pub timeline_user: User,
    pub timeline_user_id: i64,
    pub timeline_username: String,
    pub user_is_ignored: bool,
}

/// SystemInfo model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SystemInfo {
    pub time: i64,
    pub visitor_id: i64,
}

/// Thread model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Thread {
    pub contest: serde_json::Value,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: serde_json::Value,
    pub forum_id: i64,
    pub last_post: serde_json::Value,
    pub links: serde_json::Value,
    pub node_title: String,
    pub permissions: serde_json::Value,
    pub restrictions: serde_json::Value,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_closed: bool,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_starred: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: serde_json::Value,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}

/// User model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct User {
    pub balance: String,
    pub banner: String,
    pub birthday: serde_json::Value,
    pub contest_count: i64,
    pub conv_welcome_message: String,
    pub curator_titles: Vec<String>,
    pub currency: String,
    pub custom_title: String,
    pub display_banner_id: i64,
    pub display_icon_group_id: i64,
    pub edit_permissions: serde_json::Value,
    pub fields: Vec<serde_json::Value>,
    pub hold: String,
    pub is_banned: i64,
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub secret_answer_first_letter: String,
    pub secret_answer_rendered: String,
    pub self_permissions: serde_json::Value,
    pub short_link: String,
    pub trophy_count: i64,
    pub user_deposit: i64,
    pub user_email: String,
    pub user_external_authentications: Vec<serde_json::Value>,
    pub user_followers: serde_json::Value,
    pub user_following: serde_json::Value,
    pub user_group_id: i64,
    pub user_groups: Vec<serde_json::Value>,
    pub user_id: i64,
    pub user_is_followed: bool,
    pub user_is_ignored: bool,
    pub user_is_valid: bool,
    pub user_is_verified: bool,
    pub user_is_visitor: bool,
    pub user_last_seen_date: i64,
    pub user_like2_count: i64,
    pub user_like_count: i64,
    pub user_message_count: i64,
    pub user_register_date: i64,
    pub user_timezone_offset: i64,
    pub user_title: String,
    pub user_unread_conversation_count: i64,
    pub user_unread_notification_count: i64,
    pub username: String,
    pub username_html: String,
}

/// Balance model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Balance {
    pub balance: String,
    pub balance_id: i64,
    pub custom_title: serde_json::Value,
    #[serde(rename = "fullTitle")]
    pub full_title: String,
    pub merchant_id: i64,
    pub title: String,
    pub r#type: String,
    pub user_id: i64,
}

/// ConfirmationCode model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConfirmationCode {
    #[serde(rename = "codeData")]
    pub code_data: serde_json::Value,
    pub item: Item,
}

/// Discount model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Discount {
    pub category_id: i64,
    pub discount_id: i64,
    pub discount_percent: i64,
    pub discount_user_id: i64,
    pub max_price: i64,
    pub min_price: i64,
    pub user_id: i64,
}

/// Extra model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Extra {
    /// Ark. Optional. Used only if you want to upload Steam account.
    pub ark: Option<bool>,
    /// Ark Ascended. Optional. Used only if you want to upload Steam account.
    pub ark_ascended: Option<bool>,
    /// Check channels. Optional. Used only if you want to upload Telegram account.
    #[serde(rename = "checkChannels")]
    pub check_channels: Option<bool>,
    /// Check ban on Hypixel. Optional. Used only if you want to upload Minecraft account.
    #[serde(rename = "checkHypixelBan")]
    pub check_hypixel_ban: Option<bool>,
    /// Check spam. Optional. Used only if you want to upload Telegram account.
    #[serde(rename = "checkSpam")]
    pub check_spam: Option<bool>,
    /// If set, the item will be closed **item_state = closed**.
    pub close_item: Option<bool>,
    /// Code from email (in case of problems). Optional if you want to upload Supercell account.
    #[serde(rename = "confirmationCode")]
    pub confirmation_code: Option<String>,
    /// Cookie login. Optional. Used only if you want to upload TikTok account.
    pub cookie_login: Option<bool>,
    /// Cookies. Required if you want to upload Fortnite, Epic Games, Social Club, Instagram or TikTok account.
    pub cookies: Option<String>,
    /// Dota 2 MMR. Optional. Used only if you want to upload Steam account.
    pub dota2_mmr: Option<i64>,
    /// EA Games. Optional. Used only if you want to upload Steam account.
    pub ea_games: Option<bool>,
    /// Genshin Impact Primogems count. Optional. Used only if you want to upload miHoYo account.
    pub genshin_currency: Option<i64>,
    /// Honkai Star Rail Stellar Jade count. Optional. Used only if you want to upload miHoYo account.
    pub honkai_currency: Option<i64>,
    /// Login without cookies. Optional if you want to upload Instagram account.
    pub login_without_cookies: Option<bool>,
    /// Steam mafile. Optional. Used only if you want to upload Steam account.
    pub mfa_file: Option<String>,
    /// Telegram 2FA Password. Optional. Used only if you want to upload Telegram account.
    pub password: Option<String>,
    /// Proxy line format ip:port:user:pass (prioritize over proxy_id parameter).
    pub proxy: Option<String>,
    /// Region. Required if you want to upload WoT account. Optional if you want to upload miHoYo or Riot account.
    pub region: Option<String>,
    /// Service. Required if you want to upload VPN, Cinema account or gift.
    pub service: Option<String>,
    /// Supercell system. Required if you want to upload Supercell account.
    pub system: Option<String>,
    /// Telegram client. Optional. Used only if you want to upload Telegram account.
    #[serde(rename = "telegramClient")]
    pub telegram_client: Option<String>,
    /// Contents of session.json file. Optional. Used only if you want to upload Telegram account.
    #[serde(rename = "telegramJson")]
    pub telegram_json: Option<String>,
    /// The quarry. Optional. Used only if you want to upload Steam account.
    pub the_quarry: Option<bool>,
    /// Uplay Games. Optional. Used only if you want to upload Steam account.
    pub uplay_games: Option<bool>,
    /// Warframe. Optional. Used only if you want to upload Steam account.
    pub warframe: Option<bool>,
    /// Zenless Zone Zero Polychrome count. Optional. Used only if you want to upload miHoYo account.
    pub zenless_currency: Option<i64>,
}

/// Invoice model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Invoice {
    pub additional_data: String,
    pub amount: i64,
    pub comment: String,
    pub expires_at: i64,
    pub invoice_date: i64,
    pub invoice_id: i64,
    pub is_test: bool,
    pub merchant_id: i64,
    pub paid_date: i64,
    pub payer_user_id: i64,
    pub payment_id: String,
    pub resend_attempts: i64,
    pub status: String,
    pub url: String,
    pub url_callback: String,
    pub url_success: String,
    pub user_id: i64,
}

/// ItemFromList model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ItemFromList {
    pub allow_ask_discount: Option<i64>,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: Option<serde_json::Value>,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: Option<bool>,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: Option<bool>,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: Option<bool>,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: Option<bool>,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: Option<bool>,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: Option<bool>,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: Option<bool>,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: Option<bool>,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: Option<bool>,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: Option<bool>,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: Option<bool>,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: Option<bool>,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: Option<bool>,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: Option<bool>,
    pub category_id: Option<i64>,
    pub description: Option<String>,
    pub description_en: Option<String>,
    pub description_html: Option<String>,
    pub description_html_en: Option<String>,
    pub extended_guarantee: Option<i64>,
    pub guarantee: Option<bool>,
    #[serde(rename = "isIgnored")]
    pub is_ignored: Option<i64>,
    pub is_sticky: Option<i64>,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: Option<String>,
    pub item_domain: Option<String>,
    pub item_id: Option<i64>,
    pub item_origin: Option<String>,
    pub item_state: Option<String>,
    pub note_text: Option<String>,
    pub nsb: Option<i64>,
    pub price: Option<i64>,
    pub price_currency: Option<String>,
    pub published_date: Option<i64>,
    pub refreshed_date: Option<i64>,
    pub resale_item_origin: Option<String>,
    pub rub_price: Option<i64>,
    pub seller: Option<serde_json::Value>,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub title: Option<String>,
    pub title_en: Option<String>,
    pub update_stat_date: Option<i64>,
    pub view_count: Option<i64>,
}

/// ItemList model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ItemList {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(deserialize_with = "deserialize_items", default)]
    pub items: Vec<ItemFromList>,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<ItemFromList>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
}

/// Item model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Item {
    #[serde(rename = "accountLink")]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    pub account_links: Vec<serde_json::Value>,
    pub account_last_activity: i64,
    #[serde(rename = "aiPrice")]
    pub ai_price: i64,
    #[serde(rename = "aiPriceCheckDate")]
    pub ai_price_check_date: i64,
    pub allow_ask_discount: i64,
    #[serde(rename = "autoBuyPrice")]
    pub auto_buy_price: i64,
    #[serde(rename = "autoBuyPriceCheckDate")]
    pub auto_buy_price_check_date: i64,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: serde_json::Value,
    pub buyer: serde_json::Value,
    pub buyer_avatar_date: i64,
    pub buyer_display_icon_group_id: i64,
    pub buyer_uniq_banner: String,
    pub buyer_user_group_id: i64,
    #[serde(rename = "canAskDiscount")]
    pub can_ask_discount: bool,
    #[serde(rename = "canChangeEmailPassword")]
    pub can_change_email_password: bool,
    #[serde(rename = "canChangePassword")]
    pub can_change_password: bool,
    #[serde(rename = "canCheckAiPrice")]
    pub can_check_ai_price: bool,
    #[serde(rename = "canCheckAutoBuyPrice")]
    pub can_check_auto_buy_price: bool,
    #[serde(rename = "canCheckGuarantee")]
    pub can_check_guarantee: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canResellItem")]
    pub can_resell_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canShareItem")]
    pub can_share_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewAccountLoginAndTempEmail")]
    pub can_view_account_login_and_temp_email: bool,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewItemViews")]
    pub can_view_item_views: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    pub cart_price: serde_json::Value,
    pub category_id: i64,
    pub content_id: serde_json::Value,
    pub content_type: serde_json::Value,
    #[serde(rename = "copyFormatData")]
    pub copy_format_data: serde_json::Value,
    #[serde(rename = "customFields")]
    pub custom_fields: serde_json::Value,
    pub delete_date: i64,
    pub delete_reason: String,
    pub delete_user_id: i64,
    pub delete_username: String,
    pub deposit: i64,
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    #[serde(rename = "externalAuth")]
    pub external_auth: Vec<serde_json::Value>,
    #[serde(rename = "extraPrices")]
    pub extra_prices: Vec<serde_json::Value>,
    pub feedback_data: String,
    #[serde(rename = "getEmailCodeDisplayLogin")]
    pub get_email_code_display_login: serde_json::Value,
    pub guarantee: serde_json::Value,
    #[serde(rename = "imagePreviewLinks")]
    pub image_preview_links: Vec<String>,
    pub in_cart: serde_json::Value,
    pub information: String,
    pub information_en: String,
    #[serde(rename = "isBirthdayToday")]
    pub is_birthday_today: bool,
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,
    #[serde(rename = "isPersonalAccount")]
    pub is_personal_account: bool,
    #[serde(rename = "isSmallExf")]
    pub is_small_exf: bool,
    #[serde(rename = "isTrusted")]
    pub is_trusted: bool,
    pub is_fave: serde_json::Value,
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub login: String,
    #[serde(rename = "loginData")]
    pub login_data: serde_json::Value,
    pub market_custom_title: String,
    pub max_discount_percent: i64,
    #[serde(rename = "needToRequireVideoToViewLoginData")]
    pub need_to_require_video_to_view_login_data: bool,
    pub note_text: String,
    pub nsb: i64,
    pub pending_deletion_date: i64,
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    pub price_with_seller_fee: f64,
    #[serde(rename = "priceWithSellerFeeLabel")]
    pub price_with_seller_fee_label: String,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: serde_json::Value,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: bool,
    pub tags: serde_json::Value,
    pub temp_email: String,
    pub title: String,
    pub title_en: String,
    #[serde(rename = "uniqueKeyExists")]
    pub unique_key_exists: bool,
    pub update_stat_date: i64,
    pub user_allow_ask_discount: i64,
    pub view_count: i64,
    #[serde(rename = "visitorIsAuthor")]
    pub visitor_is_author: bool,
}

// API sometimes returns items as array, sometimes as object {id: item}
fn deserialize_items<'de, D>(deserializer: D) -> std::result::Result<Vec<ItemFromList>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    use serde_json::Value;

    let v = Value::deserialize(deserializer)?;
    match v {
        Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                out.push(serde_json::from_value(item).unwrap_or_default());
            }
            Ok(out)
        }
        Value::Object(map) => {
            let mut out = Vec::with_capacity(map.len());
            for (_key, item) in map {
                out.push(serde_json::from_value(item).unwrap_or_default());
            }
            Ok(out)
        }
        Value::Null => Ok(Vec::new()),
        _ => Err(de::Error::custom("expected array or object for items")),
    }
}

// ── Response wrappers ────────────────────────────────────────────

/// UsersSaResetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersSaResetResponse {
    pub success: bool,
    pub system_info: SystemInfo,
    pub waiting_time: String,
}

/// BatchExecuteResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchExecuteResponse {
    pub jobs: serde_json::Value,
}

/// CategoriesListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesListResponse {
    pub categories: Vec<serde_json::Value>,
    pub categories_total: i64,
    pub system_info: SystemInfo,
}

/// CategoriesGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesGetResponse {
    pub category: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ChatboxIndexResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexResponse {
    pub ban: serde_json::Value,
    pub commands: Vec<String>,
    pub ignore: Vec<serde_json::Value>,
    pub permissions: serde_json::Value,
    pub rooms: Vec<serde_json::Value>,
    #[serde(rename = "roomsOnline")]
    pub rooms_online: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ChatboxGetIgnoreResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetIgnoreResponse {
    pub ignored: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
}

/// ChatboxGetMessagesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetMessagesResponse {
    pub messages: Vec<ChatboxMessage>,
    pub system_info: SystemInfo,
}

/// ChatboxPostMessageResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxPostMessageResponse {
    pub message: ChatboxMessage,
    pub system_info: SystemInfo,
}

/// ChatboxEditMessageResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxEditMessageResponse {
    pub message: ChatboxMessage,
    pub system_info: SystemInfo,
}

/// ChatboxGetLeaderboardResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetLeaderboardResponse {
    pub leaderboard: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
}

/// ChatboxOnlineResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxOnlineResponse {
    pub system_info: SystemInfo,
    pub users: Vec<serde_json::Value>,
}

/// ChatboxReportReasonsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxReportReasonsResponse {
    pub reasons: Vec<String>,
    pub system_info: SystemInfo,
}

/// ThreadsClaimResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsClaimResponse {
    pub system_info: SystemInfo,
    pub thread: Thread,
}

/// ThreadsCreateContestResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsCreateContestResponse {
    pub system_info: SystemInfo,
    pub thread: Thread,
}

/// ConversationsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsListResponse {
    pub can_start: bool,
    pub conversations: Vec<Conversation>,
    pub folders: Vec<serde_json::Value>,
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ConversationsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsCreateResponse {
    pub conversation: Conversation,
    pub system_info: SystemInfo,
}

/// ConversationsUpdateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsUpdateResponse {
    pub conversation: Conversation,
    pub system_info: SystemInfo,
}

/// ConversationsMessagesGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesGetResponse {
    pub message: Conversation,
    pub system_info: SystemInfo,
}

/// ConversationsReadAllResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsReadAllResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// ConversationsSearchResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsSearchResponse {
    pub conversations: Vec<Conversation>,
    pub recipients: bool,
    pub system_info: SystemInfo,
}

/// ConversationsStartResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsStartResponse {
    pub conversation: Conversation,
    pub system_info: SystemInfo,
}

/// ConversationsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsGetResponse {
    pub conversation: Conversation,
    pub system_info: SystemInfo,
}

/// ConversationsAlertsDisableResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsAlertsDisableResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// ConversationsAlertsEnableResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsAlertsEnableResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// ConversationsMessagesListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesListResponse {
    pub links: serde_json::Value,
    pub messages: Vec<ConversationMessage>,
    pub messages_total: i64,
    pub system_info: SystemInfo,
}

/// ConversationsMessagesCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesCreateResponse {
    pub message: ConversationMessage,
    pub system_info: SystemInfo,
}

/// ConversationsMessagesEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesEditResponse {
    pub message: Conversation,
    pub system_info: SystemInfo,
}

/// ConversationsUnstarResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsUnstarResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// ConversationsStarResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsStarResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// AssetsCssResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AssetsCssResponse {
    pub contents: String,
    pub system_info: SystemInfo,
}

/// FormsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsListResponse {
    pub forms: Vec<serde_json::Value>,
    #[serde(rename = "formsPerPage")]
    pub forms_per_page: i64,
    pub page: i64,
    pub system_info: SystemInfo,
    #[serde(rename = "totalForms")]
    pub total_forms: i64,
}

/// FormsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsCreateResponse {
    pub content: serde_json::Value,
    pub message: String,
    pub system_info: SystemInfo,
}

/// ForumsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListResponse {
    pub forums: Vec<serde_json::Value>,
    pub forums_total: i64,
    pub system_info: SystemInfo,
    pub tabs: Vec<serde_json::Value>,
}

/// ForumsGetFeedOptionsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetFeedOptionsResponse {
    pub default_excluded_forums_ids: Vec<i64>,
    pub excluded_forums_ids: Vec<i64>,
    pub forums: Vec<serde_json::Value>,
    pub keywords: String,
    pub system_info: SystemInfo,
}

/// ForumsFollowedResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedResponse {
    pub forums: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
}

/// ForumsGroupedResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGroupedResponse {
    pub data: serde_json::Value,
    pub system_info: SystemInfo,
    pub tabs: Vec<serde_json::Value>,
}

/// ForumsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetResponse {
    pub forum: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ForumsFollowersResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowersResponse {
    pub system_info: SystemInfo,
    pub users: Vec<serde_json::Value>,
}

/// LinksListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LinksListResponse {
    #[serde(rename = "link-forums")]
    pub link_forums: Vec<Link>,
    #[serde(rename = "link-forums_total")]
    pub link_forums_total: i64,
    pub system_info: SystemInfo,
}

/// LinksGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LinksGetResponse {
    #[serde(rename = "link-forum")]
    pub link_forum: Link,
    pub system_info: SystemInfo,
}

/// NavigationListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NavigationListResponse {
    pub elements: Vec<serde_json::Value>,
    pub elements_count: i64,
    pub system_info: SystemInfo,
}

/// NotificationsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationsListResponse {
    pub links: serde_json::Value,
    pub notifications: Vec<Notification>,
    pub notifications_total: i64,
    pub system_info: SystemInfo,
}

/// NotificationsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationsGetResponse {
    pub notification: Notification,
    pub notification_id: i64,
    pub system_info: SystemInfo,
}

/// OAuthTokenResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OAuthTokenResponse {
    /// The access token issued by the authorization server
    pub access_token: String,
    /// The lifetime in seconds of the access token
    pub expires_in: i64,
    /// The refresh token, which can be used to obtain new access tokens
    pub refresh_token: Option<String>,
    /// The scope of the access token
    pub scope: Option<String>,
    /// The type of the token
    pub token_type: String,
}

/// PagesListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesListResponse {
    pub pages: Vec<serde_json::Value>,
    pub pages_total: i64,
    pub system_info: SystemInfo,
}

/// PagesGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesGetResponse {
    pub page: serde_json::Value,
    pub system_info: SystemInfo,
}

/// PostsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsListResponse {
    pub posts: Vec<Thread>,
    pub posts_total: i64,
    pub system_info: SystemInfo,
    pub thread: Thread,
}

/// PostsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCreateResponse {
    pub post: Post,
    pub system_info: SystemInfo,
}

/// PostsCommentsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsGetResponse {
    pub comments: Vec<PostComment>,
    pub system_info: SystemInfo,
}

/// PostsCommentsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsCreateResponse {
    pub comment: serde_json::Value,
    pub system_info: SystemInfo,
}

/// PostsCommentsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsEditResponse {
    pub comment: serde_json::Value,
    pub system_info: SystemInfo,
}

/// PostsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsGetResponse {
    pub post: Post,
    pub system_info: SystemInfo,
}

/// PostsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsEditResponse {
    pub post: Post,
    pub system_info: SystemInfo,
}

/// PostsLikesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsLikesResponse {
    pub system_info: SystemInfo,
    pub users: Vec<serde_json::Value>,
}

/// PostsReportReasonsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsReportReasonsResponse {
    pub reasons: Vec<String>,
    pub system_info: SystemInfo,
}

/// ProfilePostsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCreateResponse {
    pub profile_post: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ProfilePostsCommentsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsListResponse {
    pub comments: Vec<ProfilePostComment>,
    pub comments_total: i64,
    pub profile_post: serde_json::Value,
    pub system_info: SystemInfo,
    pub timeline_user: User,
}

/// ProfilePostsCommentsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsCreateResponse {
    pub comment: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ProfilePostsCommentsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsEditResponse {
    pub comment: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ProfilePostsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsGetResponse {
    pub profile_post: ProfilePost,
    pub system_info: SystemInfo,
}

/// ProfilePostsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsEditResponse {
    pub profile_post: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ProfilePostsCommentsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsGetResponse {
    pub comment: ProfilePostComment,
    pub system_info: SystemInfo,
}

/// ProfilePostsLikesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsLikesResponse {
    pub system_info: SystemInfo,
    pub users: Vec<serde_json::Value>,
}

/// ProfilePostsReportReasonsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsReportReasonsResponse {
    pub reasons: Vec<String>,
    pub system_info: SystemInfo,
}

/// SearchAllResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponse {
    pub data: Vec<serde_json::Value>,
    pub data_total: i64,
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
    pub users: Vec<User>,
}

/// SearchPostsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponse {
    pub data: Vec<serde_json::Value>,
    pub data_total: i64,
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
}

/// SearchProfilePostsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchProfilePostsResponse {
    pub data: Vec<serde_json::Value>,
    pub data_total: i64,
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
}

/// SearchTaggedResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponse {
    pub data: Vec<serde_json::Value>,
    pub data_total: i64,
    pub search_tags: serde_json::Value,
    pub system_info: SystemInfo,
}

/// SearchThreadsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponse {
    pub data: Vec<serde_json::Value>,
    pub data_total: i64,
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
}

/// SearchUsersResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchUsersResponse {
    pub system_info: SystemInfo,
    pub users: Vec<User>,
}

/// SearchResultsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponse {
    pub data: Vec<serde_json::Value>,
    pub data_total: i64,
    pub search_tags: serde_json::Value,
    pub system_info: SystemInfo,
}

/// TagsPopularResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsPopularResponse {
    pub system_info: SystemInfo,
    pub tags: serde_json::Value,
}

/// TagsFindResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsFindResponse {
    pub ids: Vec<i64>,
    pub system_info: SystemInfo,
    pub tags: Vec<String>,
}

/// TagsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsListResponse {
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
    pub tags: serde_json::Value,
    pub tags_total: i64,
}

/// TagsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponse {
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
    pub tag: serde_json::Value,
    pub tagged: Vec<serde_json::Value>,
    pub tagged_total: i64,
}

/// ThreadsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsListResponse {
    pub forum: serde_json::Value,
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
    pub threads: Vec<Thread>,
    pub threads_total: i64,
}

/// ThreadsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsCreateResponse {
    pub system_info: SystemInfo,
    pub thread: Thread,
}

/// ThreadsFollowedResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponse {
    pub system_info: SystemInfo,
    pub threads: Vec<serde_json::Value>,
    pub threads_total: i64,
}

/// ThreadsUnreadResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponse {
    pub data: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    pub threads: Vec<Thread>,
}

/// ThreadsRecentResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponse {
    pub data: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    pub threads: Vec<Thread>,
}

/// ThreadsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsGetResponse {
    pub system_info: SystemInfo,
    pub thread: Thread,
}

/// ThreadsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsEditResponse {
    pub system_info: SystemInfo,
    pub thread: Thread,
}

/// ThreadsBumpResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsBumpResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// ThreadsFollowersResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowersResponse {
    pub system_info: SystemInfo,
    pub users: Vec<serde_json::Value>,
}

/// ThreadsHideResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsHideResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// ThreadsNavigationResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsNavigationResponse {
    pub elements: Vec<serde_json::Value>,
    pub elements_count: i64,
    pub system_info: SystemInfo,
}

/// ThreadsPollGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsPollGetResponse {
    pub poll: serde_json::Value,
    pub system_info: SystemInfo,
}

/// UsersListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersListResponse {
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
    pub users: Vec<User>,
    pub users_total: i64,
}

/// UsersFieldsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFieldsResponse {
    pub fields: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
}

/// UsersFindResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFindResponse {
    pub system_info: SystemInfo,
    pub users: Vec<User>,
}

/// UsersIgnoredResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoredResponse {
    pub system_info: SystemInfo,
    pub users: Vec<serde_json::Value>,
}

/// UsersSecretAnswerTypesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersSecretAnswerTypesResponse {
    pub data: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
}

/// UsersGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersGetResponse {
    pub system_info: SystemInfo,
    pub user: User,
}

/// UsersAvatarUploadResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersAvatarUploadResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// UsersAvatarCropResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersAvatarCropResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// UsersBackgroundUploadResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersBackgroundUploadResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// UsersBackgroundCropResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersBackgroundCropResponse {
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// UsersClaimsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersClaimsResponse {
    pub claims: Vec<serde_json::Value>,
    pub stats: serde_json::Value,
    pub system_info: SystemInfo,
}

/// UsersFollowersResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowersResponse {
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
    pub users: Vec<serde_json::Value>,
    pub users_total: i64,
}

/// UsersFollowingsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowingsResponse {
    pub system_info: SystemInfo,
    pub users: Vec<serde_json::Value>,
    pub users_total: i64,
}

/// UsersLikesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersLikesResponse {
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub likes: serde_json::Value,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    pub system_info: SystemInfo,
    #[serde(rename = "totalLikes")]
    pub total_likes: i64,
}

/// ProfilePostsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsListResponse {
    #[serde(rename = "canPostOnProfile")]
    pub can_post_on_profile: bool,
    pub links: serde_json::Value,
    pub profile_posts: Vec<ProfilePost>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalProfilePosts")]
    pub total_profile_posts: i64,
}

/// UsersContentsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponse {
    pub data: Vec<serde_json::Value>,
    pub data_total: i64,
    pub links: serde_json::Value,
    pub system_info: SystemInfo,
    pub user: User,
}

/// UsersTrophiesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersTrophiesResponse {
    pub system_info: SystemInfo,
    pub trophies: Vec<serde_json::Value>,
}

/// AutoPaymentsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsCreateResponse {
    pub auto_payment_id: i64,
    pub message: String,
    pub status: String,
    pub system_info: SystemInfo,
}

/// AutoPaymentsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsListResponse {
    pub payments: serde_json::Value,
    pub system_info: SystemInfo,
}

/// PaymentsPayoutServicesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponse {
    pub system_info: SystemInfo,
    pub systems: Vec<serde_json::Value>,
}

/// PaymentsFeeResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsFeeResponse {
    pub calculator: serde_json::Value,
    pub commission_percentage: i64,
    #[serde(rename = "spentCurrentMonth")]
    pub spent_current_month: i64,
    pub system_info: SystemInfo,
}

/// BatchResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchResponse {
    pub jobs: serde_json::Value,
    pub system_info: Option<SystemInfo>,
}

/// CategoryBattleNetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// ManagingBulkGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingBulkGetResponse {
    pub items: Vec<serde_json::Value>,
    pub left_item_id: Vec<i64>,
    pub system_info: SystemInfo,
}

/// CartDeleteResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CartDeleteResponse {
    pub success: bool,
    pub system_info: SystemInfo,
}

/// CartAddResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CartAddResponse {
    pub success: bool,
    pub system_info: SystemInfo,
}

/// CategoryListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryListResponse {
    pub category: serde_json::Value,
    pub system_info: SystemInfo,
}

/// CategoryChatGptResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryChatGptResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// ProfileClaimsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponse {
    pub claims: Vec<serde_json::Value>,
    pub stats: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ManagingCreateClaimResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponse {
    pub system_info: serde_json::Value,
    pub thread: serde_json::Value,
}

/// PaymentsCurrencyResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponse {
    #[serde(rename = "currencyList")]
    pub currency_list: serde_json::Value,
    #[serde(rename = "lastUpdate")]
    pub last_update: i64,
    pub system_info: SystemInfo,
    #[serde(rename = "visitorCurrency")]
    pub visitor_currency: String,
}

/// CustomDiscountsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsGetResponse {
    pub discounts: Vec<Discount>,
    pub system_info: SystemInfo,
    pub total: i64,
}

/// CustomDiscountsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsCreateResponse {
    pub discount: Discount,
    pub system_info: SystemInfo,
    pub total: i64,
}

/// CustomDiscountsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsEditResponse {
    pub discounts: Vec<Discount>,
    pub system_info: SystemInfo,
    pub total: i64,
}

/// CategoryDiscordResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryDiscordResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryEaResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryEpicGamesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryEscapeFromTarkovResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEscapeFromTarkovResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryFortniteResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryGiftsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGiftsResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryHytaleResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryHytaleResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryInstagramResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryInstagramResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// PaymentsInvoiceGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceGetResponse {
    pub invoice: Invoice,
    pub system_info: SystemInfo,
}

/// PaymentsInvoiceCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceCreateResponse {
    pub invoice: Invoice,
    pub system_info: SystemInfo,
}

/// PaymentsInvoiceListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceListResponse {
    pub count: i64,
    pub invoices: Vec<Invoice>,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    pub system_info: SystemInfo,
}

/// PublishingAddResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingAddResponse {
    pub item: Item,
    pub status: String,
    pub system_info: SystemInfo,
}

/// PublishingFastSellResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingFastSellResponse {
    pub item: Item,
    #[serde(rename = "itemLink")]
    pub item_link: String,
    pub system_info: SystemInfo,
}

/// ManagingGetLetters2Response model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingGetLetters2Response {
    pub email: String,
    pub letters: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
}

/// ProfileGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileGetResponse {
    pub system_info: SystemInfo,
    pub user: User,
}

/// CategoryMihoyoResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryMinecraftResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMinecraftResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// ProxyGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProxyGetResponse {
    pub proxies: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
}

/// CategoryRiotResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryRobloxResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategorySocialClubResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySocialClubResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategorySteamResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// ManagingSteamValueResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamValueResponse {
    #[serde(rename = "appId")]
    pub app_id: Option<i64>,
    pub data: Option<serde_json::Value>,
    pub query: Option<String>,
    pub system_info: Option<SystemInfo>,
}

/// CategorySupercellResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySupercellResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryTelegramResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTelegramResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryTikTokResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTikTokResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryUplayResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// ListStatesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponse {
    pub system_info: SystemInfo,
    #[serde(rename = "userItemStates")]
    pub user_item_states: serde_json::Value,
}

/// PaymentsHistoryResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponse {
    #[serde(rename = "filterDatesDefault")]
    pub filter_dates_default: bool,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub input: serde_json::Value,
    #[serde(rename = "lastOperationId")]
    pub last_operation_id: i64,
    #[serde(rename = "nextPageHref")]
    pub next_page_href: String,
    pub page: i64,
    #[serde(rename = "pageNavLink")]
    pub page_nav_link: String,
    #[serde(rename = "pageNavParams")]
    pub page_nav_params: serde_json::Value,
    #[serde(rename = "paymentStats")]
    pub payment_stats: serde_json::Value,
    pub payments: serde_json::Value,
    #[serde(rename = "perPage")]
    pub per_page: String,
    #[serde(rename = "periodLabel")]
    pub period_label: String,
    #[serde(rename = "periodLabelPhrase")]
    pub period_label_phrase: String,
    pub system_info: SystemInfo,
}

/// CategoryVpnResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryVpnResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryWarfaceResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWarfaceResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryWotResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryWotBlitzResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponse {
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<serde_json::Value>,
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    pub was_cached: bool,
}

/// CategoryGamesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGamesResponse {
    pub games: Option<Vec<serde_json::Value>>,
    pub system_info: Option<SystemInfo>,
}

/// CategoryParamsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryParamsResponse {
    pub base_params: Option<serde_json::Value>,
    pub category: Option<serde_json::Value>,
    pub params: Option<Vec<serde_json::Value>>,
    pub system_info: Option<SystemInfo>,
}

/// ManagingGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingGetResponse {
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: bool,
    #[serde(rename = "canCancelConfirmedBuy")]
    pub can_cancel_confirmed_buy: bool,
    #[serde(rename = "canChangeOwner")]
    pub can_change_owner: bool,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    pub can_report_item: bool,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: bool,
    #[serde(rename = "canViewItemHistory")]
    pub can_view_item_history: bool,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: bool,
    #[serde(rename = "cannotBuyItemError")]
    pub cannot_buy_item_error: String,
    #[serde(rename = "faveCount")]
    pub fave_count: bool,
    #[serde(rename = "isVisibleItem")]
    pub is_visible_item: bool,
    pub item: Item,
    #[serde(rename = "itemLink")]
    pub item_link: String,
    #[serde(rename = "sameItemsCount")]
    pub same_items_count: i64,
    #[serde(rename = "sameItemsIds")]
    pub same_items_ids: Vec<i64>,
    #[serde(rename = "showToFavouritesButton")]
    pub show_to_favourites_button: bool,
    pub system_info: SystemInfo,
}

/// ManagingAiPriceResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingAiPriceResponse {
    pub price: i64,
    pub system_info: SystemInfo,
}

/// ManagingAutoBuyPriceResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingAutoBuyPriceResponse {
    pub price: i64,
    pub system_info: SystemInfo,
}

/// ManagingChangePasswordResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingChangePasswordResponse {
    pub message: Option<String>,
    pub new_password: String,
    pub status: Option<String>,
}

/// ManagingCheckGuaranteeResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCheckGuaranteeResponse {
    pub message: String,
    pub system_info: SystemInfo,
}

/// PurchasingConfirmResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PurchasingConfirmResponse {
    pub item: serde_json::Value,
    pub status: Option<String>,
    pub system_info: SystemInfo,
}

/// ManagingImageResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingImageResponse {
    pub base64: String,
    pub system_info: SystemInfo,
}

/// ManagingSteamInventoryValueResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamInventoryValueResponse {
    #[serde(rename = "appId")]
    pub app_id: Option<i64>,
    pub data: Option<serde_json::Value>,
    pub query: Option<String>,
    pub system_info: Option<SystemInfo>,
}

/// ManagingSteamGetMafileResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamGetMafileResponse {
    #[serde(rename = "maFile")]
    pub ma_file: serde_json::Value,
    pub system_info: SystemInfo,
}

/// ManagingTelegramCodeResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTelegramCodeResponse {
    pub codes: serde_json::Value,
    pub item: Item,
}

/// ManagingTempEmailPasswordResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTempEmailPasswordResponse {
    pub item: serde_json::Value,
}

/// ManagingSteamUpdateValueResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamUpdateValueResponse {
    pub item: Item,
    pub status: String,
    pub system_info: SystemInfo,
}

