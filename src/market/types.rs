//! Auto-generated types for Market API.
//! DO NOT EDIT — regenerate with `cargo run -p lolzteam-codegen`.

#![allow(unused, clippy::all)]

use serde::{Deserialize, Serialize};
use crate::models::*;

/// Parameters for `AutoPayments.Create` (POST /auto-payment).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketAutoPaymentsCreateParams {
    /// Amount to be transferred.
    pub amount: f64,
    /// Currency for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<serde_json::Value>,
    /// Day of the month for the payment. (Use "0" for the last day of the month)
    pub day: i64,
    /// Payment description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Secret answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_answer: Option<String>,
    /// Username of the payment receiver.
    pub username_receiver: String,
}

/// Parameters for `Cart.Get` (GET /cart).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCartGetParams {
    /// Accounts category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<serde_json::Value>,
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
}

/// Parameters for `Category.All` (GET /).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryAllParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
}

/// Parameters for `Category.BattleNet` (GET /battlenet).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryBattleNetParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Guarantee type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    /// List of games.
    #[serde(rename = "game[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Vec<i64>>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// Can edit BattleTag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_btag: Option<serde_json::Value>,
    /// Can edit full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changeable_fn: Option<serde_json::Value>,
    /// Real id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_id: Option<serde_json::Value>,
    /// Has disabled parent control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_control: Option<serde_json::Value>,
    /// Has no bans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_bans: Option<serde_json::Value>,
    /// Minimum balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_min: Option<i64>,
    /// Maximum balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_max: Option<i64>,
}

/// Parameters for `Category.ChatGPT` (GET /chatgpt).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryChatGptParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// List of allowed subscriptions.
    #[serde(rename = "subscription[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Vec<String>>,
    /// Length of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    /// Is auto renewal enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorenewal: Option<String>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// Has transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<serde_json::Value>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    /// List of allowed tiers.
    #[serde(rename = "openai_tier[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_tier: Option<Vec<String>>,
    /// Minimum OpenAI credit balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_balance_min: Option<i64>,
    /// Maximum OpenAI credit balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_balance_max: Option<i64>,
}

/// Parameters for `Category.Discord` (GET /discord).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryDiscordParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// Has nitro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitro: Option<serde_json::Value>,
    /// Nitro type.
    #[serde(rename = "nitro_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitro_type: Option<Vec<String>>,
    /// Length of nitro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitro_length: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitro_period: Option<String>,
    /// Minimum number of boosts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boosts_min: Option<i64>,
    /// Maximum number of boosts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boosts_max: Option<i64>,
    /// Has billing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<serde_json::Value>,
    /// Has gifts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifts: Option<serde_json::Value>,
    /// Has transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<serde_json::Value>,
    /// List of badges.
    #[serde(rename = "badge[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<Vec<String>>,
    /// List of account conditions.
    #[serde(rename = "condition[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Vec<String>>,
    /// Minimum number of chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_min: Option<i64>,
    /// Maximum number of chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_max: Option<i64>,
    /// Minimum number of subscribers in server, where account is administrator/owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin_members: Option<i64>,
    /// Maximum number of subscribers in server, where account is administrator/owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin_members: Option<i64>,
    /// Minimum number of servers, where account is administrator/owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin: Option<i64>,
    /// Maximum number of servers, where account is administrator/owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin: Option<i64>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    /// List of languages.
    #[serde(rename = "language[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    /// List of languages that won't be included.
    #[serde(rename = "not_language[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_language: Option<Vec<String>>,
    /// Has clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clans: Option<serde_json::Value>,
    /// Minimum number of clans, where account is administrator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin_clans: Option<i64>,
    /// Maximum number of clans, where account is administrator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin_clans: Option<i64>,
    /// Minimum number of clans, where account is owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_owner_clans: Option<i64>,
    /// Maximum number of clans, where account is owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_owner_clans: Option<i64>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Minimum count of servers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_servers: Option<i64>,
    /// Maximum count of servers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_servers: Option<i64>,
    /// Has two-factor authentication.
    #[serde(rename = "2fa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_2fa: Option<serde_json::Value>,
    /// Minimum number of Nitro full credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_full_credits: Option<i64>,
    /// Maximum number of Nitro full credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_full_credits: Option<i64>,
    /// Minimum number of Nitro basic credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_basic_credits: Option<i64>,
    /// Maximum number of Nitro basic credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_basic_credits: Option<i64>,
    /// Minimum number of Discord Orbs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_orbs: Option<i64>,
    /// Maximum number of Discord Orbs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_orbs: Option<i64>,
}

/// Parameters for `Category.EA` (GET /ea).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryEaParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// List of games.
    #[serde(rename = "game[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Vec<String>>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Minimum count of games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    /// Maximum count of games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    /// Minimum rank points in Apex Legends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al_rank_min: Option<i64>,
    /// Maximum rank points in Apex Legends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al_rank_max: Option<i64>,
    /// Minimum level in Apex Legends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al_level_min: Option<i64>,
    /// Maximum level in Apex Legends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al_level_max: Option<i64>,
    /// Has a ban in any game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_ban: Option<serde_json::Value>,
    /// Xbox connected to account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox_connected: Option<serde_json::Value>,
    /// Steam connected to account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam_connected: Option<serde_json::Value>,
    /// PSN connected to account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psn_connected: Option<serde_json::Value>,
    /// Name of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    /// Length of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    /// List of minimum hours played by game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played: Option<serde_json::Value>,
    /// List of maximum hours played by game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played_max: Option<serde_json::Value>,
    /// Has transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<serde_json::Value>,
}

/// Parameters for `Category.EpicGames` (GET /epicgames).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryEpicGamesParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Guarantee type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    /// List of games.
    #[serde(rename = "game[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Vec<String>>,
    /// Can change email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_email: Option<String>,
    /// Has Rocket League purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rl_purchases: Option<bool>,
    /// Minimum epic wallet balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_min: Option<f64>,
    /// Maximum epic wallet balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_max: Option<f64>,
    /// Minimum rewards balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_balance_min: Option<f64>,
    /// Maximum rewards balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_balance_max: Option<f64>,
    /// Minimum number of games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    /// Maximum number of games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// List of minimum hours played by game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played: Option<serde_json::Value>,
    /// List of maximum hours played by game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played_max: Option<serde_json::Value>,
}

/// Parameters for `Category.EscapeFromTarkov` (GET /escape-from-tarkov).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryEscapeFromTarkovParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// List of versions.
    #[serde(rename = "version[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Vec<String>>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    /// Minimum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_min: Option<i64>,
    /// Maximum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_max: Option<i64>,
    /// Access to pve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pve: Option<serde_json::Value>,
    /// Side in current wipe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
}

/// Parameters for `Category.Fortnite` (GET /fortnite).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryFortniteParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Access to market temp mail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_email: Option<String>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Guarantee type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    /// Minimum number of skins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smin: Option<i64>,
    /// Maximum number of skins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smax: Option<i64>,
    /// Minimum number of V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbmin: Option<i64>,
    /// Maximum number of V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbmax: Option<i64>,
    /// Skins.
    #[serde(rename = "skin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<Vec<String>>,
    /// Pickaxes.
    #[serde(rename = "pickaxe[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxe: Option<Vec<String>>,
    /// Gliders.
    #[serde(rename = "glider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glider: Option<Vec<String>>,
    /// Dances.
    #[serde(rename = "dance[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dance: Option<Vec<String>>,
    /// Can change email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_email: Option<String>,
    /// Platform.
    #[serde(rename = "platform[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Vec<String>>,
    /// Minimum number of shop skins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins_shop_min: Option<i64>,
    /// Maximum number of shop skins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins_shop_max: Option<i64>,
    /// Minimum number of shop pickaxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxes_shop_min: Option<i64>,
    /// Maximum number of shop pickaxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxes_shop_max: Option<i64>,
    /// Minimum number of shop dances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dances_shop_min: Option<i64>,
    /// Maximum number of shop dances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dances_shop_max: Option<i64>,
    /// Minimum number of shop gliders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gliders_shop_min: Option<i64>,
    /// Maximum number of shop gliders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gliders_shop_max: Option<i64>,
    /// Minimum total cost of all skins in the shop in V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins_shop_vbmin: Option<i64>,
    /// Maximum total cost of all skins in the shop in V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins_shop_vbmax: Option<i64>,
    /// Minimum total cost of all pickaxes in the shop in V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxes_shop_vbmin: Option<i64>,
    /// Maximum total cost of all pickaxes in the shop in V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxes_shop_vbmax: Option<i64>,
    /// Minimum total cost of all dances in the shop in V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dances_shop_vbmin: Option<i64>,
    /// Maximum total cost of all dances in the shop in V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dances_shop_vbmax: Option<i64>,
    /// Minimum total cost of all gliders in the shop in V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gliders_shop_vbmin: Option<i64>,
    /// Maximum total cost of all gliders in the shop in V-Bucks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gliders_shop_vbmax: Option<i64>,
    /// Has Battle Pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp: Option<serde_json::Value>,
    /// Minimum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmin: Option<i64>,
    /// Maximum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmax: Option<i64>,
    /// Minimum level of Battle Pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp_lmin: Option<i64>,
    /// Maximum level of Battle Pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp_lmax: Option<i64>,
    /// How old is last transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date_period: Option<String>,
    /// Has no transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_trans: Option<bool>,
    /// Can be linked to Xbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox_linkable: Option<serde_json::Value>,
    /// Can be linked to PSN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psn_linkable: Option<serde_json::Value>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Has Rocket League purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rl_purchases: Option<bool>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    /// Minimum number of available refund credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_credits_min: Option<i64>,
    /// Maximum number of available refund credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_credits_max: Option<i64>,
    /// Minimum number of pickaxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxe_min: Option<i64>,
    /// Maximum number of pickaxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickaxe_max: Option<i64>,
    /// Minimum number of dances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmin: Option<i64>,
    /// Maximum number of dances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmax: Option<i64>,
    /// Minimum number of gliders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    /// Maximum number of gliders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
}

/// Parameters for `Category.Gifts` (GET /gifts).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryGiftsParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Name of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    /// Length of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
}

/// Parameters for `Category.Hytale` (GET /hytale).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryHytaleParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// List of allowed editions.
    #[serde(rename = "edition[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<Vec<String>>,
    /// Minimum number of profiles with game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles_min: Option<i64>,
    /// Maximum number of profiles with game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles_max: Option<i64>,
}

/// Parameters for `Category.Instagram` (GET /instagram).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryInstagramParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Login by cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    /// Login without cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_without_cookies: Option<serde_json::Value>,
    /// Minimum number of followers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_min: Option<i64>,
    /// Maximum number of followers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_max: Option<i64>,
    /// Minimum number of posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_min: Option<i64>,
    /// Maximum number of posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_max: Option<i64>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
}

/// Parameters for `Category.Mihoyo` (GET /mihoyo).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryMihoyoParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Has linked email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Has linked external accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ea: Option<serde_json::Value>,
    /// Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Vec<String>>,
    /// List of disallowed regions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_region: Option<Vec<String>>,
    /// List of characters.
    #[serde(rename = "genshin_character[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_character: Option<Vec<i64>>,
    /// List of minimum constellations on characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_character_constellations: Option<serde_json::Value>,
    /// List of maximum constellations on characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_character_constellations_max: Option<serde_json::Value>,
    /// List of weapons.
    #[serde(rename = "genshin_weapon[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_weapon: Option<Vec<i64>>,
    /// Minimum number of characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_char_min: Option<i64>,
    /// Maximum number of characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_char_max: Option<i64>,
    /// Minimum number of legendary characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_legendary_min: Option<i64>,
    /// Maximum number of legendary characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_legendary_max: Option<i64>,
    /// Minimum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_level_min: Option<i64>,
    /// Maximum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_level_max: Option<i64>,
    /// Minimum number of legendary weapon characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_legendary_weapon_min: Option<i64>,
    /// Maximum number of legendary weapon characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_legendary_weapon_max: Option<i64>,
    /// Minimum number of constellations on legendary characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constellations_min: Option<i64>,
    /// Maximum number of constellations on legendary characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constellations_max: Option<i64>,
    /// Minimum number of achievements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_achievement_min: Option<i64>,
    /// Maximum number of achievements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_achievement_max: Option<i64>,
    /// Minimum number of primogems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_currency_min: Option<i64>,
    /// Maximum number of primogems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_currency_max: Option<i64>,
    /// List of characters.
    #[serde(rename = "honkai_character[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_character: Option<Vec<i64>>,
    /// List of minimum eidolons on characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_character_eidolons: Option<serde_json::Value>,
    /// List of maximum eidolons on characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_character_eidolons_max: Option<serde_json::Value>,
    /// List of weapons.
    #[serde(rename = "honkai_weapon[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_weapon: Option<Vec<i64>>,
    /// Minimum number of characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_char_min: Option<i64>,
    /// Maximum number of characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_char_max: Option<i64>,
    /// Minimum number of legendary characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_legendary_min: Option<i64>,
    /// Maximum number of legendary characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_legendary_max: Option<i64>,
    /// Minimum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_level_min: Option<i64>,
    /// Maximum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_level_max: Option<i64>,
    /// Minimum number of legendary weapon characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_legendary_weapon_min: Option<i64>,
    /// Maximum number of legendary weapon characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_legendary_weapon_max: Option<i64>,
    /// Minimum number of constellations on Honkai: Star Rail legendary characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eidolons_min: Option<i64>,
    /// Maximum number of legendary Honkai: Star Rail weapon characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eidolons_max: Option<i64>,
    /// Minimum number of achievements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_achievement_min: Option<i64>,
    /// Maximum number of achievements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_achievement_max: Option<i64>,
    /// Minimum number of Stellar Jade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_currency_min: Option<i64>,
    /// Maximum number of Stellar Jade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_currency_max: Option<i64>,
    /// List of Zenless Zone Zero characters.
    #[serde(rename = "zenless_character[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_character: Option<Vec<i64>>,
    /// List of minimum cinemas on characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_character_cinemas: Option<serde_json::Value>,
    /// List of maximum cinemas on characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_character_cinemas_max: Option<serde_json::Value>,
    /// List of Zenless Zone Zero weapons.
    #[serde(rename = "zenless_weapon[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_weapon: Option<Vec<i64>>,
    /// Minimum number of Zenless Zone Zero legendary characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_legendary_min: Option<i64>,
    /// Maximum number of Zenless Zone Zero legendary characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_legendary_max: Option<i64>,
    /// Minimum number of cinemas on Zenless Zone Zero characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinemas_min: Option<i64>,
    /// Maximum number of cinemas on Zenless Zone Zero characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinemas_max: Option<i64>,
    /// Minimum number of legendary Zenless Zone Zero weapon characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_legendary_weapon_min: Option<i64>,
    /// Maximum number of legendary Zenless Zone Zero weapon characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_legendary_weapon_max: Option<i64>,
    /// Minimum number of Zenless Zone Zero characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_char_min: Option<i64>,
    /// Maximum number of Zenless Zone Zero characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_char_max: Option<i64>,
    /// Minimum Zenless Zone Zero level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_level_min: Option<i64>,
    /// Maximum Zenless Zone Zero level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_level_max: Option<i64>,
    /// Minimum count of Zenless Zone Zero achievements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_achievement_min: Option<i64>,
    /// Maximum count of Zenless Zone Zero achievements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_achievement_max: Option<i64>,
    /// Minimum count of Zenless Zone Zero polychrome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_currency_min: Option<i64>,
    /// Maximum count of Zenless Zone Zero polychrome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_currency_max: Option<i64>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
}

/// Parameters for `Category.Minecraft` (GET /minecraft).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryMinecraftParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Name of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    /// Length of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    /// Is auto renewal enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorenewal: Option<String>,
    /// Has java edition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java: Option<serde_json::Value>,
    /// Has bedrock edition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock: Option<serde_json::Value>,
    /// Has Minecraft Dungeons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dungeons: Option<serde_json::Value>,
    /// Has Minecraft Legends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legends: Option<serde_json::Value>,
    /// Can change nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_nickname: Option<serde_json::Value>,
    /// List of capes.
    #[serde(rename = "capes[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capes: Option<Vec<String>>,
    /// Minimum number of capes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capes_min: Option<i64>,
    /// Maximum number of capes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capes_max: Option<i64>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Has active Hypixel ban.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypixel_ban: Option<serde_json::Value>,
    /// Is API enabled in Hypixel Skyblock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypixel_skyblock_api_enabled: Option<serde_json::Value>,
    /// Rank on hypixel.
    #[serde(rename = "rank_hypixel[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_hypixel: Option<Vec<String>>,
    /// Minimum number of level hypixel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_hypixel_min: Option<i64>,
    /// Maximum number of level hypixel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_hypixel_max: Option<i64>,
    /// Minimum number of achievement hypixel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achievement_hypixel_min: Option<i64>,
    /// Maximum number of achievement hypixel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achievement_hypixel_max: Option<i64>,
    /// Minimum level on Hypixel SkyBlock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_hypixel_skyblock_min: Option<i64>,
    /// Maximum level on Hypixel SkyBlock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_hypixel_skyblock_max: Option<i64>,
    /// Minimum net worth on Hypixel SkyBlock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_worth_hypixel_skyblock_min: Option<i64>,
    /// Maximum net worth on Hypixel SkyBlock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_worth_hypixel_skyblock_max: Option<i64>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    /// How old is the last login account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_hypixel: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_hypixel_period: Option<String>,
    /// Can change details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_details: Option<serde_json::Value>,
    /// Minimum number of characters in nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname_length_min: Option<i64>,
    /// Maximum number of characters in nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname_length_max: Option<i64>,
    /// Was Hypixel ban parsed by Market.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypixel_ban_parsed: Option<serde_json::Value>,
    /// Minimum number of Minecoins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecoins_min: Option<i64>,
    /// Maximum number of Minecoins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecoins_max: Option<i64>,
}

/// Parameters for `Category.Riot` (GET /riot).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryRiotParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Minimum valorant rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmin: Option<i64>,
    /// Maximum valorant rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmax: Option<i64>,
    /// Last minimum valorant rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rmin: Option<i64>,
    /// Last maximum valorant rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rmax: Option<i64>,
    /// Previous minimum rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_rmin: Option<i64>,
    /// Previous maximum rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_rmax: Option<i64>,
    /// List of weapon skins.
    #[serde(rename = "weaponSkin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_skin: Option<Vec<String>>,
    /// List of buddies.
    #[serde(rename = "buddy[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buddy: Option<Vec<String>>,
    /// List of agents.
    #[serde(rename = "agent[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Vec<String>>,
    /// List of champions.
    #[serde(rename = "champion[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub champion: Option<Vec<String>>,
    /// List of LoL skins.
    #[serde(rename = "skin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<Vec<String>>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Minimum level in Valorant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_level_min: Option<i64>,
    /// Maximum level in Valorant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_level_max: Option<i64>,
    /// Minimum level in LoL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_level_min: Option<i64>,
    /// Maximum level in LoL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_level_max: Option<i64>,
    /// Minimum inventory value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_min: Option<i64>,
    /// Maximum inventory value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_max: Option<i64>,
    /// Minimum number of Valorant points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp_min: Option<i64>,
    /// Maximum number of Valorant points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp_max: Option<i64>,
    /// Minimum number of skins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_smin: Option<i64>,
    /// Maximum number of skins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_smax: Option<i64>,
    /// List of allowed rank types.
    #[serde(rename = "valorant_rank_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_rank_type: Option<Vec<String>>,
    /// Minimum amount of agents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amin: Option<i64>,
    /// Maximum amount of agents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amax: Option<i64>,
    /// List of allowed regions in Valorant.
    #[serde(rename = "valorant_region[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_region: Option<Vec<String>>,
    /// List of disallowed regions in Valorant.
    #[serde(rename = "valorant_not_region[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_not_region: Option<Vec<String>>,
    /// List of allowed regions in LoL.
    #[serde(rename = "lol_region[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_region: Option<Vec<String>>,
    /// List of disallowed regions in LoL.
    #[serde(rename = "lol_not_region[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_not_region: Option<Vec<String>>,
    /// Has any knife.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knife: Option<bool>,
    /// Minimum number of skins in LoL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_smin: Option<i64>,
    /// Maximum number of skins in LoL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_smax: Option<i64>,
    /// Minimum number of champions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub champion_min: Option<i64>,
    /// Maximum number of champions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub champion_max: Option<i64>,
    /// Minimum win-rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_rate_min: Option<i64>,
    /// Maximum win-rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_rate_max: Option<i64>,
    /// Minimum wallet blue balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_min: Option<i64>,
    /// Maximum wallet blue balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_max: Option<i64>,
    /// Minimum wallet orange balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orange_min: Option<i64>,
    /// Maximum wallet orange balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orange_max: Option<i64>,
    /// Minimum wallet mythic balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mythic_min: Option<i64>,
    /// Maximum wallet mythic balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mythic_max: Option<i64>,
    /// Minimum wallet riot balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub riot_min: Option<i64>,
    /// Maximum wallet riot balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub riot_max: Option<i64>,
    /// Has linked email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// Minimum knifes in Valorant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_knife_min: Option<i64>,
    /// Maximum knifes in Valorant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valorant_knife_max: Option<i64>,
    /// Minimum number of Valorant Radiant Points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp_min: Option<i64>,
    /// Maximum number of Valorant Radiant Points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp_max: Option<i64>,
    /// Minimum number of Valorant free agents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fa_min: Option<i64>,
    /// Maximum number of Valorant free agents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fa_max: Option<i64>,
    /// List of allowed ranks in LoL.
    #[serde(rename = "lol_rank[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lol_rank: Option<Vec<String>>,
}

/// Parameters for `Category.Roblox` (GET /roblox).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryRobloxParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Has verified email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<serde_json::Value>,
    /// Minimum robux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux_min: Option<i64>,
    /// Maximum robux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux_max: Option<i64>,
    /// Minimum friends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_min: Option<i64>,
    /// Maximum friends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_max: Option<i64>,
    /// Minimum number of followers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_min: Option<i64>,
    /// Maximum number of followers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_max: Option<i64>,
    /// List of allowed countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    /// Name of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    /// Length of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    /// Is auto renewal enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorenewal: Option<String>,
    /// Xbox connected to account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox_connected: Option<serde_json::Value>,
    /// PSN connected to account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psn_connected: Option<serde_json::Value>,
    /// Has verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<String>,
    /// Account is age verified via documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_verified: Option<serde_json::Value>,
    /// Minimum amount of incoming robux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_robux_total_min: Option<i64>,
    /// Maximum amount of incoming robux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_robux_total_max: Option<i64>,
    /// Minimum limited items value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_price_min: Option<i64>,
    /// Maximum limited items value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_price_max: Option<i64>,
    /// Minimum total Robux cost of all game passes in popular Roblox games..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamepass_min: Option<i64>,
    /// Maximum total Robux cost of all game passes in popular Roblox games..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamepass_max: Option<i64>,
    /// Has game donations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_donations: Option<serde_json::Value>,
    /// Minimum inventory value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_min: Option<i64>,
    /// Maximum inventory value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_max: Option<i64>,
    /// Minimum UGC limited items value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ugc_limited_price_min: Option<i64>,
    /// Maximum UGC limited items value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ugc_limited_price_max: Option<i64>,
    /// Minimum credit balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_balance_min: Option<i64>,
    /// Maximum credit balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_balance_max: Option<i64>,
    /// Minimum offsale items count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsale_min: Option<i64>,
    /// Maximum offsale items count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsale_max: Option<i64>,
    /// Voice chat is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<serde_json::Value>,
    /// List of allowed age groups.
    #[serde(rename = "age_group[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_group: Option<Vec<String>>,
    /// List of disallowed age groups.
    #[serde(rename = "not_age_group[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_age_group: Option<Vec<String>>,
}

/// Parameters for `Category.SocialClub` (GET /socialclub).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategorySocialClubParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Minimum number of Social Club level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_min: Option<i64>,
    /// Maximum number of Social Club level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_max: Option<i64>,
    /// Minimum number of GTA V cash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_min: Option<i64>,
    /// Maximum number of GTA V cash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_max: Option<i64>,
    /// Minimum number of GTA V bank cash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_cash_min: Option<i64>,
    /// Maximum number of GTA V bank cash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_cash_max: Option<i64>,
    /// List of games.
    #[serde(rename = "game[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Vec<String>>,
}

/// Parameters for `Category.Steam` (GET /steam).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategorySteamParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// List of games.
    #[serde(rename = "game[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Vec<i64>>,
    /// List of minimum hours played by game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played: Option<serde_json::Value>,
    /// List of maximum hours played by game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_played_max: Option<serde_json::Value>,
    /// Guarantee type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    /// List of VAC bans by game.
    #[serde(rename = "vac[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vac: Option<Vec<i64>>,
    /// Don't check game existence while checking for vac.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vac_skip_game_check: Option<bool>,
    /// Has community ban.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rt: Option<String>,
    /// Has lifetime trade ban.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_ban: Option<serde_json::Value>,
    /// Has temporary trade limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_limit: Option<serde_json::Value>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Has 5 $ limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Value>,
    /// Has .mafile (Steam Guard Authenticator).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mafile: Option<serde_json::Value>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    /// Minimum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmin: Option<i64>,
    /// Maximum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmax: Option<i64>,
    /// Minimum rank in CS2 Matchmaking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmin: Option<i64>,
    /// Maximum rank in CS2 Matchmaking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmax: Option<i64>,
    /// Minimum rank in CS2 Wingman.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wingman_rmin: Option<i64>,
    /// Maximum rank in CS2 Wingman.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wingman_rmax: Option<i64>,
    /// Has no VAC ban.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_vac: Option<bool>,
    /// Has CS2 Matchmaking ban.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mm_ban: Option<serde_json::Value>,
    /// Minimum balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_min: Option<i64>,
    /// Maximum balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_max: Option<i64>,
    /// Game ID to check inventory price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_game: Option<i64>,
    /// Minimum inventory price for game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_min: Option<f64>,
    /// Maximum inventory price for game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_max: Option<f64>,
    /// Minimum number of friends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_min: Option<i64>,
    /// Maximum number of friends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_max: Option<i64>,
    /// Minimum number of games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    /// Maximum number of games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    /// Minimum number of wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_count_min: Option<i64>,
    /// Maximum number of wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_count_max: Option<i64>,
    /// List of medal IDs.
    #[serde(rename = "medal_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medal_id: Option<Vec<i64>>,
    /// Search for medals using "OR" instead of "AND".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medal_operator_or: Option<bool>,
    /// Minimum number of medals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medal_min: Option<i64>,
    /// Maximum number of medals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medal_max: Option<i64>,
    /// List of gifts.
    #[serde(rename = "gift[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Vec<String>>,
    /// Minimum number of gifts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_min: Option<i64>,
    /// Maximum number of gifts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_max: Option<i64>,
    /// Minimum number of recently played hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recently_hours_min: Option<i64>,
    /// Maximum number of recently played hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recently_hours_max: Option<i64>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Minimum CS2 rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_profile_rank_min: Option<i64>,
    /// Maximum CS2 rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_profile_rank_max: Option<i64>,
    /// Minimum number of Dota 2 MMR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solommr_min: Option<i64>,
    /// Maximum number of Dota 2 MMR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solommr_max: Option<i64>,
    /// Minimum number of Dota 2 games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_game_count_min: Option<i64>,
    /// Maximum number of Dota 2 games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_game_count_max: Option<i64>,
    /// Minimum number of Dota 2 wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_win_count_min: Option<i64>,
    /// Maximum number of Dota 2 wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_win_count_max: Option<i64>,
    /// Minimum number of Dota 2 behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_behavior_min: Option<i64>,
    /// Maximum number of Dota 2 behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_behavior_max: Option<i64>,
    /// Minimum FACEIT level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceit_lvl_min: Option<i64>,
    /// Maximum FACEIT level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceit_lvl_max: Option<i64>,
    /// Minimum number of Steam points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_min: Option<i64>,
    /// Maximum number of Steam points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_max: Option<i64>,
    /// Minimum number of relevant games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_gmin: Option<i64>,
    /// Maximum number of relevant games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_gmax: Option<i64>,
    /// How old is last transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date_period: Option<String>,
    /// How new is last transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date_later: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trans_date_period_later: Option<serde_json::Value>,
    /// Has no transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_trans: Option<bool>,
    /// Has transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans: Option<bool>,
    /// Minimum amount spent on gifts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifts_purchase_min: Option<f64>,
    /// Maximum amount spent on gifts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifts_purchase_max: Option<f64>,
    /// Minimum amount of refunds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds_purchase_min: Option<f64>,
    /// Minimum amount of refunds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds_purchase_max: Option<f64>,
    /// Minimum spend amount on in-game purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingame_purchase_min: Option<f64>,
    /// Maximum spend amount on in-game purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingame_purchase_max: Option<f64>,
    /// Minimum spend amount on all purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games_purchase_min: Option<f64>,
    /// Maximum spend amount on all purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games_purchase_max: Option<f64>,
    /// Minimum spend amount on all purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_min: Option<f64>,
    /// Maximum spend amount on all purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_max: Option<f64>,
    /// Has activated keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_activated_keys: Option<serde_json::Value>,
    /// Minimum Premier ELO in CS2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elo_min: Option<i64>,
    /// Maximum Premier ELO in CS2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elo_max: Option<i64>,
    /// Map for rank in CS2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_map_rank: Option<i64>,
    /// Minimum rank in CS2 on a certain map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_map_rmin: Option<i64>,
    /// Maximum rank in CS2 on a certain map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs2_map_rmax: Option<i64>,
    /// Has FACEIT account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_faceit: Option<serde_json::Value>,
    /// Minimum FACEIT level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceit_csgo_lvl_min: Option<i64>,
    /// Maximum FACEIT level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceit_csgo_lvl_max: Option<i64>,
    /// Minimum number of Rust deaths
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_deaths_min: Option<i64>,
    /// Maximum number of Rust deaths
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_deaths_max: Option<i64>,
    /// Minimum number of Rust kills
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_kills_min: Option<i64>,
    /// Maximum number of Rust kills
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_kills_max: Option<i64>,
    /// How old is last match of Dota 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_last_match_date: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2_last_match_date_period: Option<String>,
    /// Minimum number of available to collect trading cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_min: Option<i64>,
    /// Maximum number of available to collect trading cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_max: Option<i64>,
    /// Minimum number of available games with available to collect trading cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_games_min: Option<i64>,
    /// Maximum number of available games with available to collect trading cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_games_max: Option<i64>,
    /// Ignore inventory value if game has VAC ban.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_vac_inv: Option<bool>,
}

/// Parameters for `Category.Supercell` (GET /supercell).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategorySupercellParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Guarantee type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<i64>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<serde_json::Value>,
    /// Minimum Brawl Stars level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_level_min: Option<i64>,
    /// Maximum Brawl Stars level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_level_max: Option<i64>,
    /// Minimum number of Brawl Stars trophies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_cup_min: Option<i64>,
    /// Maximum number of Brawl Stars trophies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_cup_max: Option<i64>,
    /// Minimum number of Brawl Stars wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_wins_min: Option<i64>,
    /// Maximum number of Brawl Stars wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_wins_max: Option<i64>,
    /// Has Brawl Pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawl_pass: Option<serde_json::Value>,
    /// List of brawlers.
    #[serde(rename = "brawler[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawler: Option<Vec<String>>,
    /// Minimum number of brawlers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawlers_min: Option<i64>,
    /// Maximum number of brawlers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawlers_max: Option<i64>,
    /// Minimum number of legendary brawlers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendary_brawlers_min: Option<i64>,
    /// Maximum number of legendary brawlers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendary_brawlers_max: Option<i64>,
    /// Minimum Clash Royale level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_level_min: Option<i64>,
    /// Maximum Clash Royale level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_level_max: Option<i64>,
    /// Minimum number of Clash Royale trophies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_cup_min: Option<i64>,
    /// Maximum number of Clash Royale trophies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_cup_max: Option<i64>,
    /// Minimum number of Clash Royale wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_wins_min: Option<i64>,
    /// Maximum number of Clash Royale wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_wins_max: Option<i64>,
    /// Minimum King level in Clash Royale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub king_level_min: Option<i64>,
    /// Maximum King level in Clash Royale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub king_level_max: Option<i64>,
    /// Has Royale Pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale_pass: Option<serde_json::Value>,
    /// Minimum Clash of Clans level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_level_min: Option<i64>,
    /// Maximum Clash of Clans level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_level_max: Option<i64>,
    /// Minimum number of Clash of Clans trophies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_cup_min: Option<i64>,
    /// Maximum number of Clash of Clans trophies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_cup_max: Option<i64>,
    /// Minimum number of Clash of Clans wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_wins_min: Option<i64>,
    /// Maximum number of Clash of Clans wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_wins_max: Option<i64>,
    /// Has Battle Pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_pass: Option<serde_json::Value>,
    /// Minimum total heroes level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_heroes_level_min: Option<i64>,
    /// Maximum total heroes level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_heroes_level_max: Option<i64>,
    /// Minimum total troops level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_troops_level_min: Option<i64>,
    /// Maximum total troops level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_troops_level_max: Option<i64>,
    /// Minimum total spells level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spells_level_min: Option<i64>,
    /// Maximum total spells level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spells_level_max: Option<i64>,
    /// Minimum total builder village heroes level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_builder_heroes_level_min: Option<i64>,
    /// Maximum total builder village heroes level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_builder_heroes_level_max: Option<i64>,
    /// Minimum total builder village troops level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_builder_troops_level_min: Option<i64>,
    /// Maximum total builder village troops level count in Clash of Clans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_builder_troops_level_max: Option<i64>,
    /// Minimum level of town hall.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town_hall_level_min: Option<i64>,
    /// Maximum level of town hall.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town_hall_level_max: Option<i64>,
    /// Minimum level of builder hall.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_hall_level_min: Option<i64>,
    /// Maximum level of builder hall.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_hall_level_max: Option<i64>,
    /// Minimum number of builder hall cups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_hall_cup_min: Option<i64>,
    /// Maximum number of builder hall cups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_hall_cup_max: Option<i64>,
    /// Minimum account creation year (e.g. 2023).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_year_min: Option<i64>,
    /// Maximum account creation year (e.g. 2024).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_year_max: Option<i64>,
}

/// Parameters for `Category.Telegram` (GET /telegram).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryTelegramParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Has a spam ban.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam: Option<serde_json::Value>,
    /// Has a cloud password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<serde_json::Value>,
    /// Has a premium subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<String>,
    /// When premium subscription will be active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration: Option<i64>,
    /// In what notation is time measured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration_period: Option<String>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Minimum number of channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_channels: Option<i64>,
    /// Maximum number of channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_channels: Option<i64>,
    /// Minimum number of chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_chats: Option<i64>,
    /// Maximum number of chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chats: Option<i64>,
    /// Minimum number of conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_conversations: Option<i64>,
    /// Maximum number of conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conversations: Option<i64>,
    /// Minimum number of channels, where account is administrator/owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin: Option<i64>,
    /// Maximum number of channels, where account is administrator/owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin: Option<i64>,
    /// Minimum number of subscribers in channel, where account is administrator/owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_admin_sub: Option<i64>,
    /// Maximum number of subscribers in channel, where account is administrator/owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_admin_sub: Option<i64>,
    /// Minimum number of digits in ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dig_min: Option<i64>,
    /// Maximum number of digits in ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dig_max: Option<i64>,
    /// Minimum number of contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_contacts: Option<i64>,
    /// Maximum number of contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contacts: Option<i64>,
    /// Minimum number of Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stars: Option<i64>,
    /// Maximum number of Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stars: Option<i64>,
    /// Birthday was X time before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_period: Option<String>,
    /// Birthday was X time after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_after: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_after_period: Option<String>,
    /// Minimum ID of account, will be rounded down till nearest 10k. Available if your balance is higher than 100000 RUB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_id: Option<i64>,
    /// Maximum ID of account, will be rounded down till nearest 10k. Available if your balance is higher than 100000 RUB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<i64>,
    /// Allow geo spam block in search with spam=no.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_geo_spamblock: Option<bool>,
    /// Minimum number of Telegram gifts on account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_gifts: Option<i64>,
    /// Maximum number of Telegram gifts on account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gifts: Option<i64>,
    /// Minimum number of Telegram NFT gifts on account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_nft_gifts: Option<i64>,
    /// Maximum number of Telegram NFT gifts on account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nft_gifts: Option<i64>,
    /// Minimum value of all Stars gifts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_gifts_stars: Option<i64>,
    /// Maximum value of all Stars gifts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gifts_stars: Option<i64>,
    /// Minimum value of all Stars gifts after convert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_gifts_convert_stars: Option<i64>,
    /// Maximum value of all Stars gifts after convert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gifts_convert_stars: Option<i64>,
    /// List of allowed DC ID.
    #[serde(rename = "dc_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_id: Option<Vec<i64>>,
    /// List of disallowed DC ID.
    #[serde(rename = "not_dc_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_dc_id: Option<Vec<i64>>,
    /// Has linked email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Minimum number of bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bots: Option<i64>,
    /// Maximum number of bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bots: Option<i64>,
    /// Minimum active users in bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bot_active_users: Option<i64>,
    /// Maximum active users in bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bot_active_users: Option<i64>,
}

/// Parameters for `Category.TikTok` (GET /tiktok).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryTikTokParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
    /// Minimum number of followers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_min: Option<i64>,
    /// Maximum number of followers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_max: Option<i64>,
    /// Minimum number of posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_min: Option<i64>,
    /// Maximum number of posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_max: Option<i64>,
    /// Minimum number of likes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_min: Option<i64>,
    /// Maximum number of likes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_max: Option<i64>,
    /// Minimum number of coins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coins_min: Option<i64>,
    /// Maximum number of coins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coins_max: Option<i64>,
    /// Login by cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_login: Option<serde_json::Value>,
    /// Has verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<String>,
    /// Has linked email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Parameters for `Category.Uplay` (GET /uplay).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryUplayParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// List of games.
    #[serde(rename = "game[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Vec<String>>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Minimum count of games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<i64>,
    /// Maximum count of games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<i64>,
    /// Name of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    /// Length of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    /// Minimum level in Tom Clancy's Rainbow Six Siege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_level_min: Option<i64>,
    /// Maximum level in Tom Clancy's Rainbow Six Siege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_level_max: Option<i64>,
    /// Minimum rank points in Tom Clancy's Rainbow Six Siege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_rank_min: Option<i64>,
    /// Maximum rank points in Tom Clancy's Rainbow Six Siege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_rank_max: Option<i64>,
    /// Minimum count of operators in Tom Clancy's Rainbow Six Siege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_operators_min: Option<i64>,
    /// Maximum count of operators in Tom Clancy's Rainbow Six Siege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_operators_max: Option<i64>,
    /// Is account banned in Tom Clancy's Rainbow Six Siege
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_ban: Option<serde_json::Value>,
    /// Minimum number of skins in Tom Clancy's Rainbow Six Siege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_smin: Option<i64>,
    /// Maximum number of skins in Tom Clancy's Rainbow Six Siege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_smax: Option<i64>,
    /// List of weapon skins in Tom Clancy's Rainbow Six Siege.
    #[serde(rename = "r6_skin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_skin: Option<Vec<String>>,
    /// List of operators in Tom Clancy's Rainbow Six Siege.
    #[serde(rename = "r6_operator[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r6_operator: Option<Vec<String>>,
    /// Xbox connected to account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox_connected: Option<serde_json::Value>,
    /// PSN connected to account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psn_connected: Option<serde_json::Value>,
    /// Steam connected to account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam_connected: Option<serde_json::Value>,
    /// Minimum balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_min: Option<f64>,
    /// Maximum balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_max: Option<f64>,
    /// Has transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<serde_json::Value>,
    /// How old is the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_period: Option<String>,
}

/// Parameters for `Category.Vpn` (GET /vpn).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryVpnParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// List of allowed VPN services.
    #[serde(rename = "service[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<String>>,
    /// Length of subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_length: Option<i64>,
    /// In what notation is time measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>,
    /// Is auto renewal enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorenewal: Option<String>,
}

/// Parameters for `Category.Warface` (GET /warface).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryWarfaceParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Minimum rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_min: Option<i64>,
    /// Maximum rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_max: Option<i64>,
    /// Minimum bonus rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_rank_min: Option<i64>,
    /// Maximum bonus rank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_rank_max: Option<i64>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Minimum amount of Kredits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kredits_min: Option<i64>,
    /// Maximum amount of Kredits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kredits_max: Option<i64>,
    /// Minimum total donated Kredits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_kredits_min: Option<i64>,
    /// Maximum total donated Kredits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_kredits_max: Option<i64>,
}

/// Parameters for `Category.Wot` (GET /world-of-tanks).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryWotParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Minimum number of battles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battles_min: Option<i64>,
    /// Maximum number of battles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battles_max: Option<i64>,
    /// Minimum number of gold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold_min: Option<i64>,
    /// Maximum number of gold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold_max: Option<i64>,
    /// Minimum number of silver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silver_min: Option<i64>,
    /// Maximum number of silver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silver_max: Option<i64>,
    /// Minimum number of top tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_min: Option<i64>,
    /// Maximum number of top tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_max: Option<i64>,
    /// Minimum number of premium tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prem_min: Option<i64>,
    /// Maximum number of premium tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prem_max: Option<i64>,
    /// Minimum number of top premium tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_prem_min: Option<i64>,
    /// Maximum number of top premium tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_prem_max: Option<i64>,
    /// Minimum number of wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_pmin: Option<i64>,
    /// Maximum number of wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_pmax: Option<i64>,
    /// List of tanks.
    #[serde(rename = "tank[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tank: Option<Vec<i64>>,
    /// Region.
    #[serde(rename = "region[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Vec<String>>,
    /// Exclude region.
    #[serde(rename = "not_region[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_region: Option<Vec<String>>,
    /// Has a premium subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<String>,
    /// When premium subscription will be active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration: Option<i64>,
    /// In what notation is time measured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration_period: Option<String>,
    /// Has clan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan: Option<String>,
    /// List of allowed clan role.
    #[serde(rename = "clan_role[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_role: Option<Vec<String>>,
    /// List of disallowed clan role.
    #[serde(rename = "not_clan_role[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_clan_role: Option<Vec<String>>,
    /// Minimum number of gold in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_gold_min: Option<i64>,
    /// Maximum number of gold in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_gold_max: Option<i64>,
    /// Minimum number of credits in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_credits_min: Option<i64>,
    /// Maximum number of credits in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_credits_max: Option<i64>,
    /// Minimum number of crystal in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_crystal_min: Option<i64>,
    /// Maximum number of crystal in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_crystal_max: Option<i64>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
}

/// Parameters for `Category.WotBlitz` (GET /wot-blitz).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCategoryWotBlitzParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_tag_id: Option<Vec<i64>>,
    /// List of tag ids (Tag list is available via **GET /me**).
    #[serde(rename = "public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_tag_id: Option<Vec<i64>>,
    /// List of tag ids that won't be included (Tag list is available via **GET /me**).
    #[serde(rename = "not_public_tag_id[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_public_tag_id: Option<Vec<i64>>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Search accounts of user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Currency in which the cost of the account will be searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Has email login data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<bool>,
    /// Email provider.
    #[serde(rename = "email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_provider: Option<Vec<String>>,
    /// Email provider.
    #[serde(rename = "not_email_provider[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_email_provider: Option<String>,
    /// Parse same item ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_same_item_ids: Option<bool>,
    /// Email type.
    #[serde(rename = "email_type[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<Vec<String>>,
    /// Domain of native/autoreg email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    /// Has linked mobile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// Number of days the account has been offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daybreak: Option<i64>,
    /// Minimum number of battles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battles_min: Option<i64>,
    /// Maximum number of battles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battles_max: Option<i64>,
    /// Minimum number of gold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold_min: Option<i64>,
    /// Maximum number of gold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold_max: Option<i64>,
    /// Minimum number of silver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silver_min: Option<i64>,
    /// Maximum number of silver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silver_max: Option<i64>,
    /// Minimum number of top tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_min: Option<i64>,
    /// Maximum number of top tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_max: Option<i64>,
    /// Minimum number of premium tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prem_min: Option<i64>,
    /// Maximum number of premium tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prem_max: Option<i64>,
    /// Minimum number of top premium tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_prem_min: Option<i64>,
    /// Maximum number of top premium tanks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_prem_max: Option<i64>,
    /// Minimum number of wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_pmin: Option<i64>,
    /// Maximum number of wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_pmax: Option<i64>,
    /// List of tanks.
    #[serde(rename = "tank[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tank: Option<Vec<i64>>,
    /// Region.
    #[serde(rename = "region[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Vec<String>>,
    /// Exclude region.
    #[serde(rename = "not_region[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_region: Option<Vec<String>>,
    /// Has a premium subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<String>,
    /// When premium subscription will be active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration: Option<i64>,
    /// In what notation is time measured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_expiration_period: Option<String>,
    /// Has clan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan: Option<String>,
    /// List of allowed clan role.
    #[serde(rename = "clan_role[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_role: Option<Vec<String>>,
    /// List of disallowed clan role.
    #[serde(rename = "not_clan_role[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_clan_role: Option<Vec<String>>,
    /// Minimum number of gold in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_gold_min: Option<i64>,
    /// Maximum number of gold in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_gold_max: Option<i64>,
    /// Minimum number of credits in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_credits_min: Option<i64>,
    /// Maximum number of credits in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_credits_max: Option<i64>,
    /// Minimum number of crystal in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_crystal_min: Option<i64>,
    /// Maximum number of crystal in clan treasure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_crystal_max: Option<i64>,
    /// List of allowed countries.
    #[serde(rename = "country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// List of disallowed countries.
    #[serde(rename = "not_country[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_country: Option<Vec<String>>,
}

/// Parameters for `CustomDiscounts.Create` (POST /custom-discounts).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCustomDiscountsCreateParams {
    pub category_id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<serde_json::Value>,
    /// Discount percent to apply.
    pub discount_percent: f64,
    /// Maximum accounts price for which the discount applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f64>,
    /// Minimum accounts price for which the discount applies.
    pub min_price: f64,
    /// User ID.
    pub user_id: i64,
}

/// Parameters for `CustomDiscounts.Edit` (PUT /custom-discounts).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketCustomDiscountsEditParams {
    /// ID of the discount to edit.
    pub discount_id: i64,
    /// Discount percent to apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percent: Option<f64>,
    /// Maximum price for which the discount applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f64>,
    /// Minimum price for which the discount applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_price: Option<f64>,
}

/// Parameters for `Imap.Create` (POST /imap).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketImapCreateParams {
    /// Domain to delete IMAP configuration for.
    pub domain: String,
    /// IMAP server address.
    pub imap_server: String,
    /// IMAP server port.
    pub port: i64,
    /// Whether to use a secure connection.
    pub secure: bool,
}

/// Parameters for `List.Download` (GET /user/{type}/download).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketListDownloadParams {
    /// Format of the downloaded accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Custom format string for download. (Required if **format** is set to **custom**)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_format: Option<String>,
    /// Accounts category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<serde_json::Value>,
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Account status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    /// Delete reason. (Only if **show** is set to **deleted**)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_reason: Option<String>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Username of buyer. (If **show** is **paid**)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Start date for filtering by publication date.
    #[serde(rename = "published_startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_start_date: Option<String>,
    /// End date for filtering by publication date.
    #[serde(rename = "published_endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_end_date: Option<String>,
    /// Enable filtering by publication date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_published_date: Option<bool>,
    /// Start date for filtering by buyer operation date.
    #[serde(rename = "paid_startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_start_date: Option<String>,
    /// End date for filtering by buyer operation date.
    #[serde(rename = "paid_endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_end_date: Option<String>,
    /// Enable filtering by buyer operation date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_buyer_operation_date: Option<bool>,
    /// Start date for filtering by deletion date.
    #[serde(rename = "delete_startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_start_date: Option<String>,
    /// End date for filtering by deletion date.
    #[serde(rename = "delete_endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_end_date: Option<String>,
    /// Enable filtering by deletion date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_delete_date: Option<bool>,
}

/// Parameters for `List.Favorites` (GET /fave).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketListFavoritesParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Account status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
}

/// Parameters for `List.Orders` (GET /user/orders).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketListOrdersParams {
    /// User id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Accounts category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<serde_json::Value>,
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Account status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// Login.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
}

/// Parameters for `List.User` (GET /user/items).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketListUserParams {
    /// User id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Accounts category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<serde_json::Value>,
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Account status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    /// Delete reason. (Only if **show** is set to **deleted**)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_reason: Option<String>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// Login.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
    /// Username of buyer. (If **show** is **paid**)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Start date for filtering by publication date.
    #[serde(rename = "published_startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_start_date: Option<String>,
    /// End date for filtering by publication date.
    #[serde(rename = "published_endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_end_date: Option<String>,
    /// Enable filtering by publication date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_published_date: Option<bool>,
    /// Start date for filtering by buyer operation date.
    #[serde(rename = "paid_startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_start_date: Option<String>,
    /// End date for filtering by buyer operation date.
    #[serde(rename = "paid_endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_end_date: Option<String>,
    /// Enable filtering by buyer operation date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_buyer_operation_date: Option<bool>,
    /// Start date for filtering by deletion date.
    #[serde(rename = "delete_startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_start_date: Option<String>,
    /// End date for filtering by deletion date.
    #[serde(rename = "delete_endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_end_date: Option<String>,
    /// Enable filtering by deletion date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_delete_date: Option<bool>,
}

/// Parameters for `List.Viewed` (GET /viewed).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketListViewedParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Account status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    /// The word or words contained in the account title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// List of account origins.
    #[serde(rename = "origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// List of account origins that won't be included.
    #[serde(rename = "not_origin[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_origin: Option<Vec<String>>,
    /// Order by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb: Option<bool>,
    /// Sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_by_me: Option<bool>,
    /// Not sold before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<bool>,
    /// Not sold by me before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb_by_me: Option<bool>,
}

/// Parameters for `Managing.Edit` (PUT /{item_id}/edit).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketManagingEditParams {
    /// Allow users to ask discount for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_discount: Option<bool>,
    /// Using currency for amount. Required if you are trying to change price field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<serde_json::Value>,
    /// Account public description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Email login data (email:password format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    /// Email type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    /// Account private information (visible only for buyer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<String>,
    /// Account origin. Where did you get it from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_origin: Option<String>,
    /// Current price of account in your currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// Using proxy id for account checking. See GET or POST /proxy to get or edit proxy list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_id: Option<i64>,
    /// Title of account. If **title** specified and **title_en** is empty, **title_en** will be automatically translated to English language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// English title of account. If **title_en** specified and **title** is empty, **title** will be automatically translated to Russian language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}

/// Parameters for `Managing.GetLetters2` (GET /letters2).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketManagingGetLetters2Params {
    /// Email login data (email:password format). Required if both *email* and *password* are not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_password: Option<String>,
    /// Email. Required if *email_password* is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Password. Required if *email_password* is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Number of letters to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Parameters for `Managing.SteamValue` (GET /steam-value).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketManagingSteamValueParams {
    /// Link or id of account. Can be [**https://lzt.market/{item-id}/**, **https://steamcommunity.com/id/{steam-name}**, **https://steamcommunity.com/profiles/{steam-id}**, **{steam-id}**].
    pub link: String,
    /// Application id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    /// Currency in which the inventory value will be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<serde_json::Value>,
    /// Ignore cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cache: Option<bool>,
}

/// Parameters for `Payments.History` (GET /user/payments).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketPaymentsHistoryParams {
    /// Type of operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Minimal price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<i64>,
    /// Maximum price of account (Inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<i64>,
    /// Currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<serde_json::Value>,
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Id of the operation from which the result begins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id_lt: Option<i64>,
    /// Username of user, which receive money from you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<String>,
    /// Username of user, which sent money to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// Returns payments that are done via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_api: Option<bool>,
    /// Start date of operation (RFC 3339 date format).
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// End date of operation (RFC 3339 date format).
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Wallet, which used for money payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    /// Comment for money transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Display hold operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hold: Option<bool>,
    /// Display payment stats for selected period (outgoing value, incoming value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_payment_stats: Option<bool>,
}

/// Parameters for `Payments.Invoice.Create` (POST /invoice).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketPaymentsInvoiceCreateParams {
    /// Additional information for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<String>,
    /// Invoice amount.
    pub amount: f64,
    /// Comment to the invoice.
    pub comment: String,
    /// Currency that will be used to create the invoice.
    pub currency: serde_json::Value,
    /// Create a test invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_test: Option<bool>,
    /// Invoice lifetime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<f64>,
    /// Merchant ID.
    pub merchant_id: i64,
    /// Payment ID in your system (must be unique within the merchant / invoices).
    pub payment_id: String,
    /// Telegram User ID for which the invoice was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_telegram_id: Option<i64>,
    /// Telegram Username (including @) for which the invoice was created (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_telegram_username: Option<String>,
    /// Callback url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_callback: Option<String>,
    /// URL to redirect to after successful payment.
    pub url_success: String,
}

/// Parameters for `Payments.Invoice.List` (GET /invoice/list).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketPaymentsInvoiceListParams {
    /// The number of the page to display results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Currency of the created invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<serde_json::Value>,
    /// Status of the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Invoice amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Merchant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<i64>,
}

/// Parameters for `Payments.Payout` (POST /balance/payout).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketPaymentsPayoutParams {
    pub amount: f64,
    pub currency: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fee: Option<bool>,
    pub payment_system: String,
    pub wallet: String,
}

/// Parameters for `Payments.Transfer` (POST /balance/transfer).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketPaymentsTransferParams {
    /// Amount to send in your currency.
    pub amount: i64,
    /// Transfer comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Using currency for amount.
    pub currency: serde_json::Value,
    /// Hold length option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_length_option: Option<String>,
    /// Hold length value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_length_value: Option<i64>,
    /// Is the deal happening on Telegram? 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_deal: Option<bool>,
    /// Telegram username of the user you are dialoguing with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_username: Option<String>,
    /// Hold transfer or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_hold: Option<bool>,
    /// User id of receiver. If **user_id** specified, **username** is not required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Username of receiver. If **username** specified, **user_id** is not required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Parameters for `Profile.Edit` (PUT /me).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketProfileEditParams {
    /// Usernames who can transfer market accounts to you. Separate values with a comma.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_accept_accounts: Option<Vec<String>>,
    /// Clear Telegram data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_telegram_client: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<serde_json::Value>,
    /// Telegram api hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_api_hash: Option<String>,
    /// Telegram api id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_api_id: Option<String>,
    /// Telegram app version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_app_version: Option<String>,
    /// Telegram device model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_device_model: Option<String>,
    /// Telegram lang code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_lang_code: Option<String>,
    /// Telegram lang pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_lang_pack: Option<String>,
    /// Telegram system lang code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_system_lang_code: Option<String>,
    /// Telegram system version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_system_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<serde_json::Value>,
}

/// Parameters for `Proxy.Add` (POST /proxy).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketProxyAddParams {
    /// Proxy ip or host. Required if **proxy_row** is not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ip: Option<String>,
    /// Proxy password. Required if **proxy_row** is not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_pass: Option<String>,
    /// Proxy port. Required if **proxy_row** is not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<i64>,
    /// Proxy list in String format ip:port:user:pass. Each proxy must be start with new line (use \r\n separator)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_row: Option<String>,
    /// Proxy username. Required if **proxy_row** is not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_user: Option<String>,
}

/// Parameters for `Publishing.Add` (POST /item/add).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketPublishingAddParams {
    /// Allow users to ask discount for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_discount: Option<bool>,
    /// Accounts category.
    pub category_id: i64,
    pub currency: serde_json::Value,
    /// Account public description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Required if a **category** is one of list of Required email login data categories. Email login data (email:password format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    /// Email type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    /// Guarantee type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_guarantee: Option<i64>,
    /// Get temporary email if not required by category. Available for Supercell, Fortnite and Epic Games categories.
    #[serde(rename = "forceTempEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_temp_email: Option<bool>,
    /// Required if a **category** is one of list of Required email login data categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_email_login_data: Option<bool>,
    /// Account private information (visible only for buyer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<String>,
    /// Account origin. Where did you get it from.
    pub item_origin: String,
    /// Current price of account in your currency.
    pub price: f64,
    /// Proxy id that will be used to check account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_proxy: Option<serde_json::Value>,
    /// Put item id, if you are trying to resell item. This is useful to pass temporary email from reselling item to new item. You will get same temporary email from reselling account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resell_item_id: Option<i64>,
    /// Title of account. If **title** specified and **title_en** is empty, **title_en** will be automatically translated to English language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// English title of account. If **title_en** specified and **title** is empty, **title** will be automatically translated to Russian language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}

/// Parameters for `Publishing.Check` (POST /{item_id}/goods/check).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketPublishingCheckParams {
    /// Required if a **category** is one of list of Required email login data categories. Email login data (email:password format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    /// Email type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Extra>,
    /// Required if a **category** is one of list of Required email login data categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_email_login_data: Option<bool>,
    /// Account login (or email).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// Account login data (login:password format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    /// Account password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Set this parameter to **true** so that the Market will take a random proxy from its pool for each of your requests.
    /// Otherwise, if this parameter is set to **false** or not set, the Market will take a specific proxy from its pool, which is predefined for each item.
    /// > This parameter only works with proxies from the Market pool. If you want to use your own proxies, use the proxy_id or extra[proxy] parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_proxy: Option<bool>,
    /// Put if you are trying to resell an account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resell_item_id: Option<i64>,
}

/// Parameters for `Publishing.External` (POST /{item_id}/external-account).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketPublishingExternalParams {
    /// Cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    /// Email login data (email:password format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    /// Account login data (login:password format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// External account type.
    pub r#type: String,
}

/// Parameters for `Publishing.FastSell` (POST /item/fast-sell).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketPublishingFastSellParams {
    /// Allow users to ask discount for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_discount: Option<bool>,
    /// Accounts category.
    pub category_id: i64,
    /// Using currency.
    pub currency: serde_json::Value,
    /// Account public description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Required if a **category** is one of list of Required email login data categories. Email login data (email:password format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_login_data: Option<String>,
    /// Email type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    /// Guarantee type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_guarantee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Extra>,
    /// Required if a **category** is one of list of Required email login data categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_email_login_data: Option<bool>,
    /// Account private information (visible only for buyer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<String>,
    /// Account origin. Where did you get it from.
    pub item_origin: String,
    /// Account login (or email).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// Account login data (login:password format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    /// Account password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Current price of account in your currency.
    pub price: f64,
    /// Proxy id that will be used to check account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_proxy: Option<serde_json::Value>,
    /// Title of account. If **title** specified and **title_en** is empty, **title_en** will be automatically translated to English language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// English title of account. If **title_en** specified and **title** is empty, **title** will be automatically translated to Russian language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}

