// generated — do not edit



#![allow(unused, clippy::all)]

use crate::client::ApiClient;
use crate::error::Result;
use crate::models::*;
use crate::market::types::*;

/// Market API methods.
impl crate::market::MarketApi {

    // ── Account publishing ──

    /// Add Account
    /// `POST /item/add`
    pub async fn publishing_add(
        &self,
        params: MarketPublishingAddParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_ask_discount {
            body.insert("allow_ask_discount".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("category_id".into(), serde_json::to_value(&params.category_id).unwrap_or_default());
        body.insert("currency".into(), serde_json::to_value(&params.currency).unwrap_or_default());
        if let Some(v) = &params.description {
            body.insert("description".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.email_login_data {
            body.insert("email_login_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.email_type {
            body.insert("email_type".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.extended_guarantee {
            body.insert("extended_guarantee".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.force_temp_email {
            body.insert("forceTempEmail".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.has_email_login_data {
            body.insert("has_email_login_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.information {
            body.insert("information".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("item_origin".into(), serde_json::to_value(&params.item_origin).unwrap_or_default());
        body.insert("price".into(), serde_json::to_value(&params.price).unwrap_or_default());
        if let Some(v) = &params.proxy_id {
            body.insert("proxy_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.random_proxy {
            body.insert("random_proxy".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.resell_item_id {
            body.insert("resell_item_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title {
            body.insert("title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title_en {
            body.insert("title_en".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/item/add",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Check Account Details
    /// `POST /{item_id}/goods/check`
    pub async fn publishing_check(
        &self,
        item_id: i64,
        params: MarketPublishingCheckParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.email_login_data {
            body.insert("email_login_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.email_type {
            body.insert("email_type".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.extra {
            body.insert("extra".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.has_email_login_data {
            body.insert("has_email_login_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.login {
            body.insert("login".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.login_password {
            body.insert("login_password".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.password {
            body.insert("password".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.random_proxy {
            body.insert("random_proxy".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.resell_item_id {
            body.insert("resell_item_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/{item_id}/goods/check"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Add an External Account
    /// `POST /{item_id}/external-account`
    pub async fn publishing_external(
        &self,
        item_id: i64,
        params: MarketPublishingExternalParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.cookies {
            body.insert("cookies".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.email_login_data {
            body.insert("email_login_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.login {
            body.insert("login".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("type".into(), serde_json::to_value(&params.r#type).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/{item_id}/external-account"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Fast Account Upload
    /// `POST /item/fast-sell`
    pub async fn publishing_fast_sell(
        &self,
        params: MarketPublishingFastSellParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_ask_discount {
            body.insert("allow_ask_discount".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("category_id".into(), serde_json::to_value(&params.category_id).unwrap_or_default());
        body.insert("currency".into(), serde_json::to_value(&params.currency).unwrap_or_default());
        if let Some(v) = &params.description {
            body.insert("description".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.email_login_data {
            body.insert("email_login_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.email_type {
            body.insert("email_type".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.extended_guarantee {
            body.insert("extended_guarantee".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.extra {
            body.insert("extra".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.has_email_login_data {
            body.insert("has_email_login_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.information {
            body.insert("information".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("item_origin".into(), serde_json::to_value(&params.item_origin).unwrap_or_default());
        if let Some(v) = &params.login {
            body.insert("login".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.login_password {
            body.insert("login_password".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.password {
            body.insert("password".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("price".into(), serde_json::to_value(&params.price).unwrap_or_default());
        if let Some(v) = &params.proxy_id {
            body.insert("proxy_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.random_proxy {
            body.insert("random_proxy".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title {
            body.insert("title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title_en {
            body.insert("title_en".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/item/fast-sell",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }


    // ── Account purchasing ──

    /// Check Account
    /// `POST /{item_id}/check-account`
    pub async fn purchasing_check(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/check-account"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Confirm Buy
    /// `POST /{item_id}/confirm-buy`
    pub async fn purchasing_confirm(
        &self,
        item_id: i64,
        balance_id: Option<i64>,
        price: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &balance_id {
            body.insert("balance_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &price {
            body.insert("price".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/{item_id}/confirm-buy"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Cancel Discount Request
    /// `DELETE /{item_id}/discount`
    pub async fn purchasing_discount_cancel(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/{item_id}/discount"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Discount Request
    /// `POST /{item_id}/discount`
    pub async fn purchasing_discount_request(
        &self,
        item_id: i64,
        discount_price: f64,
        message: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("discount_price".into(), serde_json::to_value(&discount_price).unwrap_or_default());
        if let Some(v) = &message {
            body.insert("message".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/{item_id}/discount"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Fast Buy Account
    /// `POST /{item_id}/fast-buy`
    pub async fn purchasing_fast_buy(
        &self,
        item_id: i64,
        balance_id: Option<i64>,
        price: Option<f64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &balance_id {
            body.insert("balance_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &price {
            body.insert("price".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/{item_id}/fast-buy"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }


    // ── Accounts list ──

    /// Download Accounts Data
    /// `GET /user/{type}/download`
    pub async fn list_download(
        &self,
        r#type: String,
        params: MarketListDownloadParams,
    ) -> Result<serde_json::Value> {
        let _path_type = r#type;
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.format {
            query.push(("format", v.to_string()));
        }
        if let Some(v) = &params.custom_format {
            query.push(("custom_format", v.to_string()));
        }
        if let Some(v) = &params.category_id {
            query.push(("category_id", v.to_string()));
        }
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.show {
            query.push(("show", v.to_string()));
        }
        if let Some(v) = &params.delete_reason {
            query.push(("delete_reason", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.username {
            query.push(("username", v.to_string()));
        }
        if let Some(v) = &params.published_start_date {
            query.push(("published_startDate", v.to_string()));
        }
        if let Some(v) = &params.published_end_date {
            query.push(("published_endDate", v.to_string()));
        }
        if let Some(v) = &params.filter_by_published_date {
            query.push(("filter_by_published_date", v.to_string()));
        }
        if let Some(v) = &params.paid_start_date {
            query.push(("paid_startDate", v.to_string()));
        }
        if let Some(v) = &params.paid_end_date {
            query.push(("paid_endDate", v.to_string()));
        }
        if let Some(v) = &params.filter_by_buyer_operation_date {
            query.push(("filter_by_buyer_operation_date", v.to_string()));
        }
        if let Some(v) = &params.delete_start_date {
            query.push(("delete_startDate", v.to_string()));
        }
        if let Some(v) = &params.delete_end_date {
            query.push(("delete_endDate", v.to_string()));
        }
        if let Some(v) = &params.filter_by_delete_date {
            query.push(("filter_by_delete_date", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/user/{_path_type}/download"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get All Favourites Accounts
    /// `GET /fave`
    pub async fn list_favorites(
        &self,
        params: MarketListFavoritesParams,
    ) -> Result<ItemList> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.show {
            query.push(("show", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        self.client.request(
            "get",
            "/fave",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get All Purchased Accounts
    /// `GET /user/orders`
    pub async fn list_orders(
        &self,
        params: MarketListOrdersParams,
    ) -> Result<ItemList> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.category_id {
            query.push(("category_id", v.to_string()));
        }
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.show {
            query.push(("show", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.login {
            query.push(("login", v.to_string()));
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        self.client.request(
            "get",
            "/user/orders",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get User Items States
    /// `GET /user/item-states`
    pub async fn list_states(
        &self,
        user_id: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &user_id {
            query.push(("user_id", v.to_string()));
        }
        self.client.request(
            "get",
            "/user/item-states",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get All User Accounts
    /// `GET /user/items`
    pub async fn list_user(
        &self,
        params: MarketListUserParams,
    ) -> Result<ItemList> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.category_id {
            query.push(("category_id", v.to_string()));
        }
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.show {
            query.push(("show", v.to_string()));
        }
        if let Some(v) = &params.delete_reason {
            query.push(("delete_reason", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.login {
            query.push(("login", v.to_string()));
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.username {
            query.push(("username", v.to_string()));
        }
        if let Some(v) = &params.published_start_date {
            query.push(("published_startDate", v.to_string()));
        }
        if let Some(v) = &params.published_end_date {
            query.push(("published_endDate", v.to_string()));
        }
        if let Some(v) = &params.filter_by_published_date {
            query.push(("filter_by_published_date", v.to_string()));
        }
        if let Some(v) = &params.paid_start_date {
            query.push(("paid_startDate", v.to_string()));
        }
        if let Some(v) = &params.paid_end_date {
            query.push(("paid_endDate", v.to_string()));
        }
        if let Some(v) = &params.filter_by_buyer_operation_date {
            query.push(("filter_by_buyer_operation_date", v.to_string()));
        }
        if let Some(v) = &params.delete_start_date {
            query.push(("delete_startDate", v.to_string()));
        }
        if let Some(v) = &params.delete_end_date {
            query.push(("delete_endDate", v.to_string()));
        }
        if let Some(v) = &params.filter_by_delete_date {
            query.push(("filter_by_delete_date", v.to_string()));
        }
        self.client.request(
            "get",
            "/user/items",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get All Viewed Accounts
    /// `GET /viewed`
    pub async fn list_viewed(
        &self,
        params: MarketListViewedParams,
    ) -> Result<ItemList> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.show {
            query.push(("show", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        self.client.request(
            "get",
            "/viewed",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Accounts managing ──

    /// Get AI Price
    /// `GET /{item_id}/ai-price`
    pub async fn managing_ai_price(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/{item_id}/ai-price"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Auto Bump
    /// `POST /{item_id}/auto-bump`
    pub async fn managing_auto_bump(
        &self,
        item_id: i64,
        hour: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("hour".into(), serde_json::to_value(&hour).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/{item_id}/auto-bump"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Disable Auto Bump
    /// `DELETE /{item_id}/auto-bump`
    pub async fn managing_auto_bump_disable(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/{item_id}/auto-bump"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Auto Buy Price
    /// `GET /{item_id}/auto-buy-price`
    pub async fn managing_auto_buy_price(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/{item_id}/auto-buy-price"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Bulk Get Accounts
    /// `POST /bulk/items`
    pub async fn managing_bulk_get(
        &self,
        item_id: Option<Vec<serde_json::Value>>,
        parse_same_item_ids: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &item_id {
            body.insert("item_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &parse_same_item_ids {
            body.insert("parse_same_item_ids".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/bulk/items",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Bump Account
    /// `POST /{item_id}/bump`
    pub async fn managing_bump(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/bump"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Change Password
    /// `POST /{item_id}/change-password`
    pub async fn managing_change_password(
        &self,
        item_id: i64,
        cancel: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &cancel {
            body.insert("_cancel".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/{item_id}/change-password"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Check Guarantee
    /// `POST /{item_id}/check-guarantee`
    pub async fn managing_check_guarantee(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/check-guarantee"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Close Account
    /// `POST /{item_id}/close`
    pub async fn managing_close(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/close"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Create Claim
    /// `POST /claims`
    pub async fn managing_create_claim(
        &self,
        item_id: serde_json::Value,
        post_body: String,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("item_id".into(), serde_json::to_value(&item_id).unwrap_or_default());
        body.insert("post_body".into(), serde_json::to_value(&post_body).unwrap_or_default());
        self.client.request(
            "post",
            "/claims",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Decline Video Recording Request
    /// `POST /{item_id}/decline-video-recording`
    pub async fn managing_decline_video_recording(
        &self,
        item_id: i64,
        i_voluntarily_and_with_full_awareness_of_my_actions_waive_any_claims_regarding_this_item: bool,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("i_voluntarily_and_with_full_awareness_of_my_actions_waive_any_claims_regarding_this_item".into(), serde_json::to_value(&i_voluntarily_and_with_full_awareness_of_my_actions_waive_any_claims_regarding_this_item).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/{item_id}/decline-video-recording"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Edit Account
    /// `PUT /{item_id}/edit`
    pub async fn managing_edit(
        &self,
        item_id: i64,
        params: MarketManagingEditParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_ask_discount {
            body.insert("allow_ask_discount".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.currency {
            body.insert("currency".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.description {
            body.insert("description".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.email_login_data {
            body.insert("email_login_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.email_type {
            body.insert("email_type".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.information {
            body.insert("information".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.item_origin {
            body.insert("item_origin".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.price {
            body.insert("price".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.proxy_id {
            body.insert("proxy_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title {
            body.insert("title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title_en {
            body.insert("title_en".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "put",
            &format!("/{item_id}/edit"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Email Confirmation Code
    /// `GET /{item_id}/email-code`
    pub async fn managing_email_code(
        &self,
        item_id: i64,
    ) -> Result<ConfirmationCode> {
        self.client.request(
            "get",
            &format!("/{item_id}/email-code"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Favorite
    /// `POST /{item_id}/star`
    pub async fn managing_favorite(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/star"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Account
    /// `GET /{item_id}`
    pub async fn managing_get(
        &self,
        item_id: i64,
        parse_same_item_ids: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/{item_id}"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Email Letters
    /// `GET /letters2`
    pub async fn managing_get_letters2(
        &self,
        params: MarketManagingGetLetters2Params,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.email_password {
            query.push(("email_password", v.to_string()));
        }
        if let Some(v) = &params.email {
            query.push(("email", v.to_string()));
        }
        if let Some(v) = &params.password {
            query.push(("password", v.to_string()));
        }
        if let Some(v) = &params.limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            "/letters2",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Account Image
    /// `GET /{item_id}/image`
    pub async fn managing_image(
        &self,
        item_id: i64,
        r#type: String,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        query.push(("type", r#type.to_string()));
        self.client.request(
            "get",
            &format!("/{item_id}/image"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Note
    /// `POST /{item_id}/note-save`
    pub async fn managing_note(
        &self,
        item_id: i64,
        text: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &text {
            body.insert("text".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/{item_id}/note-save"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Open Account
    /// `POST /{item_id}/open`
    pub async fn managing_open(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/open"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Add a Public Tag
    /// `POST /{item_id}/public-tag`
    pub async fn managing_public_tag(
        &self,
        item_id: i64,
        tag_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("tag_id".into(), serde_json::to_value(&tag_id).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/{item_id}/public-tag"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Remove a Public Tag
    /// `DELETE /{item_id}/public-tag`
    pub async fn managing_public_untag(
        &self,
        item_id: i64,
        tag_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/{item_id}/public-tag"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Cancel Guarantee
    /// `POST /{item_id}/refuse-guarantee`
    pub async fn managing_refuse_guarantee(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/refuse-guarantee"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Add Mafile
    /// `POST /{item_id}/mafile`
    pub async fn managing_steam_add_mafile(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/mafile"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Mafile
    /// `GET /{item_id}/mafile`
    pub async fn managing_steam_get_mafile(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/{item_id}/mafile"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Remove Mafile
    /// `DELETE /{item_id}/mafile`
    pub async fn managing_steam_remove_mafile(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/{item_id}/mafile"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Account Steam Inventory Value
    /// `GET /{item_id}/inventory-value`
    pub async fn managing_steam_inventory_value(
        &self,
        item_id: i64,
        app_id: Option<i64>,
        currency: Option<serde_json::Value>,
        ignore_cache: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &app_id {
            query.push(("app_id", v.to_string()));
        }
        if let Some(v) = &currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &ignore_cache {
            query.push(("ignore_cache", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/{item_id}/inventory-value"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Mafile Confirmation Code
    /// `GET /{item_id}/guard-code`
    pub async fn managing_steam_mafile_code(
        &self,
        item_id: i64,
    ) -> Result<ConfirmationCode> {
        self.client.request(
            "get",
            &format!("/{item_id}/guard-code"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Steam HTML
    /// `GET /{item_id}/steam-preview`
    pub async fn managing_steam_preview(
        &self,
        item_id: i64,
        r#type: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &r#type {
            query.push(("type", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/{item_id}/steam-preview"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Confirm SDA
    /// `POST /{item_id}/confirm-sda`
    pub async fn managing_steam_sda(
        &self,
        item_id: i64,
        id: Option<i64>,
        nonce: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &id {
            body.insert("id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &nonce {
            body.insert("nonce".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/{item_id}/confirm-sda"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Update Inventory Value
    /// `POST /{item_id}/update-inventory`
    pub async fn managing_steam_update_value(
        &self,
        item_id: i64,
        all: Option<bool>,
        app_id: Option<i64>,
        authorize: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &all {
            body.insert("all".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &app_id {
            body.insert("app_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &authorize {
            body.insert("authorize".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/{item_id}/update-inventory"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Steam Inventory Value
    /// `GET /steam-value`
    pub async fn managing_steam_value(
        &self,
        params: MarketManagingSteamValueParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        query.push(("link", params.link.to_string()));
        if let Some(v) = &params.app_id {
            query.push(("app_id", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.ignore_cache {
            query.push(("ignore_cache", v.to_string()));
        }
        self.client.request(
            "get",
            "/steam-value",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Stick Account
    /// `POST /{item_id}/stick`
    pub async fn managing_stick(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/stick"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Add a Tag
    /// `POST /{item_id}/tag`
    pub async fn managing_tag(
        &self,
        item_id: i64,
        tag_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("tag_id".into(), serde_json::to_value(&tag_id).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/{item_id}/tag"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Telegram Confirmation Code
    /// `GET /{item_id}/telegram-login-code`
    pub async fn managing_telegram_code(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/{item_id}/telegram-login-code"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Telegram Reset Auth
    /// `POST /{item_id}/telegram-reset-authorizations`
    pub async fn managing_telegram_reset_auth(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/{item_id}/telegram-reset-authorizations"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Temp Email Password
    /// `GET /{item_id}/temp-email-password`
    pub async fn managing_temp_email_password(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/{item_id}/temp-email-password"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Change Account Owner
    /// `POST /{item_id}/change-owner`
    pub async fn managing_transfer(
        &self,
        item_id: i64,
        secret_answer: String,
        username: String,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("secret_answer".into(), serde_json::to_value(&secret_answer).unwrap_or_default());
        body.insert("username".into(), serde_json::to_value(&username).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/{item_id}/change-owner"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Unfavorite
    /// `DELETE /{item_id}/star`
    pub async fn managing_unfavorite(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/{item_id}/star"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Unstick Account
    /// `DELETE /{item_id}/stick`
    pub async fn managing_unstick(
        &self,
        item_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/{item_id}/stick"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Remove a Tag
    /// `DELETE /{item_id}/tag`
    pub async fn managing_untag(
        &self,
        item_id: i64,
        tag_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/{item_id}/tag"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Delete Account
    /// `DELETE /{item_id}`
    pub async fn manging_delete(
        &self,
        item_id: i64,
        reason: String,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/{item_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Claims
    /// `GET /claims`
    pub async fn profile_claims(
        &self,
        r#type: Option<String>,
        claim_state: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &r#type {
            query.push(("type", v.to_string()));
        }
        if let Some(v) = &claim_state {
            query.push(("claim_state", v.to_string()));
        }
        self.client.request(
            "get",
            "/claims",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Batch requests ──

    /// Batch
    /// `POST /batch`
    pub async fn batch(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            "/batch",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Cart ──

    /// Add Item to Cart
    /// `POST /cart`
    pub async fn cart_add(
        &self,
        item_id: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("item_id".into(), serde_json::to_value(&item_id).unwrap_or_default());
        self.client.request(
            "post",
            "/cart",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Item From Cart
    /// `DELETE /cart`
    pub async fn cart_delete(
        &self,
        item_id: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/cart",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Cart Items
    /// `GET /cart`
    pub async fn cart_get(
        &self,
        params: MarketCartGetParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.category_id {
            query.push(("category_id", v.to_string()));
        }
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        self.client.request(
            "get",
            "/cart",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Categories ──

    /// Get Category Games
    /// `GET /{categoryName}/games`
    pub async fn category_games(
        &self,
        category_name: String,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/{category_name}/games"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Categories
    /// `GET /category`
    pub async fn category_list(
        &self,
        top_queries: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &top_queries {
            query.push(("top_queries", v.to_string()));
        }
        self.client.request(
            "get",
            "/category",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Category Search Params
    /// `GET /{categoryName}/params`
    pub async fn category_params(
        &self,
        category_name: String,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/{category_name}/params"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Category Search ──

    /// Get Last Accounts
    /// `GET /`
    pub async fn category_all(
        &self,
        params: MarketCategoryAllParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        self.client.request(
            "get",
            "/",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// BattleNet
    /// `GET /battlenet`
    pub async fn category_battle_net(
        &self,
        params: MarketCategoryBattleNetParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.eg {
            query.push(("eg", v.to_string()));
        }
        if let Some(v) = &params.game {
            for item in v {
                query.push(("game[]", item.to_string()));
            }
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.edit_btag {
            query.push(("edit_btag", v.to_string()));
        }
        if let Some(v) = &params.changeable_fn {
            query.push(("changeable_fn", v.to_string()));
        }
        if let Some(v) = &params.real_id {
            query.push(("real_id", v.to_string()));
        }
        if let Some(v) = &params.parent_control {
            query.push(("parent_control", v.to_string()));
        }
        if let Some(v) = &params.no_bans {
            query.push(("no_bans", v.to_string()));
        }
        if let Some(v) = &params.balance_min {
            query.push(("balance_min", v.to_string()));
        }
        if let Some(v) = &params.balance_max {
            query.push(("balance_max", v.to_string()));
        }
        self.client.request(
            "get",
            "/battlenet",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// ChatGPT
    /// `GET /chatgpt`
    pub async fn category_chat_gpt(
        &self,
        params: MarketCategoryChatGptParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.subscription {
            for item in v {
                query.push(("subscription[]", item.to_string()));
            }
        }
        if let Some(v) = &params.subscription_length {
            query.push(("subscription_length", v.to_string()));
        }
        if let Some(v) = &params.subscription_period {
            query.push(("subscription_period", v.to_string()));
        }
        if let Some(v) = &params.autorenewal {
            query.push(("autorenewal", v.to_string()));
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.transactions {
            query.push(("transactions", v.to_string()));
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        if let Some(v) = &params.openai_tier {
            for item in v {
                query.push(("openai_tier[]", item.to_string()));
            }
        }
        if let Some(v) = &params.openai_balance_min {
            query.push(("openai_balance_min", v.to_string()));
        }
        if let Some(v) = &params.openai_balance_max {
            query.push(("openai_balance_max", v.to_string()));
        }
        self.client.request(
            "get",
            "/chatgpt",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Discord
    /// `GET /discord`
    pub async fn category_discord(
        &self,
        params: MarketCategoryDiscordParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.nitro {
            query.push(("nitro", v.to_string()));
        }
        if let Some(v) = &params.nitro_type {
            for item in v {
                query.push(("nitro_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.nitro_length {
            query.push(("nitro_length", v.to_string()));
        }
        if let Some(v) = &params.nitro_period {
            query.push(("nitro_period", v.to_string()));
        }
        if let Some(v) = &params.boosts_min {
            query.push(("boosts_min", v.to_string()));
        }
        if let Some(v) = &params.boosts_max {
            query.push(("boosts_max", v.to_string()));
        }
        if let Some(v) = &params.billing {
            query.push(("billing", v.to_string()));
        }
        if let Some(v) = &params.gifts {
            query.push(("gifts", v.to_string()));
        }
        if let Some(v) = &params.transactions {
            query.push(("transactions", v.to_string()));
        }
        if let Some(v) = &params.badge {
            for item in v {
                query.push(("badge[]", item.to_string()));
            }
        }
        if let Some(v) = &params.condition {
            for item in v {
                query.push(("condition[]", item.to_string()));
            }
        }
        if let Some(v) = &params.chat_min {
            query.push(("chat_min", v.to_string()));
        }
        if let Some(v) = &params.chat_max {
            query.push(("chat_max", v.to_string()));
        }
        if let Some(v) = &params.min_admin_members {
            query.push(("min_admin_members", v.to_string()));
        }
        if let Some(v) = &params.max_admin_members {
            query.push(("max_admin_members", v.to_string()));
        }
        if let Some(v) = &params.min_admin {
            query.push(("min_admin", v.to_string()));
        }
        if let Some(v) = &params.max_admin {
            query.push(("max_admin", v.to_string()));
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        if let Some(v) = &params.language {
            for item in v {
                query.push(("language[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_language {
            for item in v {
                query.push(("not_language[]", item.to_string()));
            }
        }
        if let Some(v) = &params.clans {
            query.push(("clans", v.to_string()));
        }
        if let Some(v) = &params.min_admin_clans {
            query.push(("min_admin_clans", v.to_string()));
        }
        if let Some(v) = &params.max_admin_clans {
            query.push(("max_admin_clans", v.to_string()));
        }
        if let Some(v) = &params.min_owner_clans {
            query.push(("min_owner_clans", v.to_string()));
        }
        if let Some(v) = &params.max_owner_clans {
            query.push(("max_owner_clans", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.min_servers {
            query.push(("min_servers", v.to_string()));
        }
        if let Some(v) = &params.max_servers {
            query.push(("max_servers", v.to_string()));
        }
        if let Some(v) = &params.f_2fa {
            query.push(("2fa", v.to_string()));
        }
        if let Some(v) = &params.min_full_credits {
            query.push(("min_full_credits", v.to_string()));
        }
        if let Some(v) = &params.max_full_credits {
            query.push(("max_full_credits", v.to_string()));
        }
        if let Some(v) = &params.min_basic_credits {
            query.push(("min_basic_credits", v.to_string()));
        }
        if let Some(v) = &params.max_basic_credits {
            query.push(("max_basic_credits", v.to_string()));
        }
        if let Some(v) = &params.min_orbs {
            query.push(("min_orbs", v.to_string()));
        }
        if let Some(v) = &params.max_orbs {
            query.push(("max_orbs", v.to_string()));
        }
        self.client.request(
            "get",
            "/discord",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// EA (Origin)
    /// `GET /ea`
    pub async fn category_ea(
        &self,
        params: MarketCategoryEaParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.game {
            for item in v {
                query.push(("game[]", item.to_string()));
            }
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.gmin {
            query.push(("gmin", v.to_string()));
        }
        if let Some(v) = &params.gmax {
            query.push(("gmax", v.to_string()));
        }
        if let Some(v) = &params.al_rank_min {
            query.push(("al_rank_min", v.to_string()));
        }
        if let Some(v) = &params.al_rank_max {
            query.push(("al_rank_max", v.to_string()));
        }
        if let Some(v) = &params.al_level_min {
            query.push(("al_level_min", v.to_string()));
        }
        if let Some(v) = &params.al_level_max {
            query.push(("al_level_max", v.to_string()));
        }
        if let Some(v) = &params.has_ban {
            query.push(("has_ban", v.to_string()));
        }
        if let Some(v) = &params.xbox_connected {
            query.push(("xbox_connected", v.to_string()));
        }
        if let Some(v) = &params.steam_connected {
            query.push(("steam_connected", v.to_string()));
        }
        if let Some(v) = &params.psn_connected {
            query.push(("psn_connected", v.to_string()));
        }
        if let Some(v) = &params.subscription {
            query.push(("subscription", v.to_string()));
        }
        if let Some(v) = &params.subscription_length {
            query.push(("subscription_length", v.to_string()));
        }
        if let Some(v) = &params.subscription_period {
            query.push(("subscription_period", v.to_string()));
        }
        if let Some(v) = &params.hours_played {
            query.push(("hours_played", v.to_string()));
        }
        if let Some(v) = &params.hours_played_max {
            query.push(("hours_played_max", v.to_string()));
        }
        if let Some(v) = &params.transactions {
            query.push(("transactions", v.to_string()));
        }
        self.client.request(
            "get",
            "/ea",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Epic Games
    /// `GET /epicgames`
    pub async fn category_epic_games(
        &self,
        params: MarketCategoryEpicGamesParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.eg {
            query.push(("eg", v.to_string()));
        }
        if let Some(v) = &params.game {
            for item in v {
                query.push(("game[]", item.to_string()));
            }
        }
        if let Some(v) = &params.change_email {
            query.push(("change_email", v.to_string()));
        }
        if let Some(v) = &params.rl_purchases {
            query.push(("rl_purchases", v.to_string()));
        }
        if let Some(v) = &params.balance_min {
            query.push(("balance_min", v.to_string()));
        }
        if let Some(v) = &params.balance_max {
            query.push(("balance_max", v.to_string()));
        }
        if let Some(v) = &params.rewards_balance_min {
            query.push(("rewards_balance_min", v.to_string()));
        }
        if let Some(v) = &params.rewards_balance_max {
            query.push(("rewards_balance_max", v.to_string()));
        }
        if let Some(v) = &params.gmin {
            query.push(("gmin", v.to_string()));
        }
        if let Some(v) = &params.gmax {
            query.push(("gmax", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.hours_played {
            query.push(("hours_played", v.to_string()));
        }
        if let Some(v) = &params.hours_played_max {
            query.push(("hours_played_max", v.to_string()));
        }
        self.client.request(
            "get",
            "/epicgames",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Escape from Tarkov
    /// `GET /escape-from-tarkov`
    pub async fn category_escape_from_tarkov(
        &self,
        params: MarketCategoryEscapeFromTarkovParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.region {
            query.push(("region", v.to_string()));
        }
        if let Some(v) = &params.version {
            for item in v {
                query.push(("version[]", item.to_string()));
            }
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        if let Some(v) = &params.level_min {
            query.push(("level_min", v.to_string()));
        }
        if let Some(v) = &params.level_max {
            query.push(("level_max", v.to_string()));
        }
        if let Some(v) = &params.pve {
            query.push(("pve", v.to_string()));
        }
        if let Some(v) = &params.side {
            query.push(("side", v.to_string()));
        }
        self.client.request(
            "get",
            "/escape-from-tarkov",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Fortnite
    /// `GET /fortnite`
    pub async fn category_fortnite(
        &self,
        params: MarketCategoryFortniteParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.temp_email {
            query.push(("temp_email", v.to_string()));
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.eg {
            query.push(("eg", v.to_string()));
        }
        if let Some(v) = &params.smin {
            query.push(("smin", v.to_string()));
        }
        if let Some(v) = &params.smax {
            query.push(("smax", v.to_string()));
        }
        if let Some(v) = &params.vbmin {
            query.push(("vbmin", v.to_string()));
        }
        if let Some(v) = &params.vbmax {
            query.push(("vbmax", v.to_string()));
        }
        if let Some(v) = &params.skin {
            for item in v {
                query.push(("skin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.pickaxe {
            for item in v {
                query.push(("pickaxe[]", item.to_string()));
            }
        }
        if let Some(v) = &params.glider {
            for item in v {
                query.push(("glider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.dance {
            for item in v {
                query.push(("dance[]", item.to_string()));
            }
        }
        if let Some(v) = &params.change_email {
            query.push(("change_email", v.to_string()));
        }
        if let Some(v) = &params.platform {
            for item in v {
                query.push(("platform[]", item.to_string()));
            }
        }
        if let Some(v) = &params.skins_shop_min {
            query.push(("skins_shop_min", v.to_string()));
        }
        if let Some(v) = &params.skins_shop_max {
            query.push(("skins_shop_max", v.to_string()));
        }
        if let Some(v) = &params.pickaxes_shop_min {
            query.push(("pickaxes_shop_min", v.to_string()));
        }
        if let Some(v) = &params.pickaxes_shop_max {
            query.push(("pickaxes_shop_max", v.to_string()));
        }
        if let Some(v) = &params.dances_shop_min {
            query.push(("dances_shop_min", v.to_string()));
        }
        if let Some(v) = &params.dances_shop_max {
            query.push(("dances_shop_max", v.to_string()));
        }
        if let Some(v) = &params.gliders_shop_min {
            query.push(("gliders_shop_min", v.to_string()));
        }
        if let Some(v) = &params.gliders_shop_max {
            query.push(("gliders_shop_max", v.to_string()));
        }
        if let Some(v) = &params.skins_shop_vbmin {
            query.push(("skins_shop_vbmin", v.to_string()));
        }
        if let Some(v) = &params.skins_shop_vbmax {
            query.push(("skins_shop_vbmax", v.to_string()));
        }
        if let Some(v) = &params.pickaxes_shop_vbmin {
            query.push(("pickaxes_shop_vbmin", v.to_string()));
        }
        if let Some(v) = &params.pickaxes_shop_vbmax {
            query.push(("pickaxes_shop_vbmax", v.to_string()));
        }
        if let Some(v) = &params.dances_shop_vbmin {
            query.push(("dances_shop_vbmin", v.to_string()));
        }
        if let Some(v) = &params.dances_shop_vbmax {
            query.push(("dances_shop_vbmax", v.to_string()));
        }
        if let Some(v) = &params.gliders_shop_vbmin {
            query.push(("gliders_shop_vbmin", v.to_string()));
        }
        if let Some(v) = &params.gliders_shop_vbmax {
            query.push(("gliders_shop_vbmax", v.to_string()));
        }
        if let Some(v) = &params.bp {
            query.push(("bp", v.to_string()));
        }
        if let Some(v) = &params.lmin {
            query.push(("lmin", v.to_string()));
        }
        if let Some(v) = &params.lmax {
            query.push(("lmax", v.to_string()));
        }
        if let Some(v) = &params.bp_lmin {
            query.push(("bp_lmin", v.to_string()));
        }
        if let Some(v) = &params.bp_lmax {
            query.push(("bp_lmax", v.to_string()));
        }
        if let Some(v) = &params.last_trans_date {
            query.push(("last_trans_date", v.to_string()));
        }
        if let Some(v) = &params.last_trans_date_period {
            query.push(("last_trans_date_period", v.to_string()));
        }
        if let Some(v) = &params.no_trans {
            query.push(("no_trans", v.to_string()));
        }
        if let Some(v) = &params.xbox_linkable {
            query.push(("xbox_linkable", v.to_string()));
        }
        if let Some(v) = &params.psn_linkable {
            query.push(("psn_linkable", v.to_string()));
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.rl_purchases {
            query.push(("rl_purchases", v.to_string()));
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        if let Some(v) = &params.refund_credits_min {
            query.push(("refund_credits_min", v.to_string()));
        }
        if let Some(v) = &params.refund_credits_max {
            query.push(("refund_credits_max", v.to_string()));
        }
        if let Some(v) = &params.pickaxe_min {
            query.push(("pickaxe_min", v.to_string()));
        }
        if let Some(v) = &params.pickaxe_max {
            query.push(("pickaxe_max", v.to_string()));
        }
        if let Some(v) = &params.dmin {
            query.push(("dmin", v.to_string()));
        }
        if let Some(v) = &params.dmax {
            query.push(("dmax", v.to_string()));
        }
        if let Some(v) = &params.gmin {
            query.push(("gmin", v.to_string()));
        }
        if let Some(v) = &params.gmax {
            query.push(("gmax", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/fortnite",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Gifts
    /// `GET /gifts`
    pub async fn category_gifts(
        &self,
        params: MarketCategoryGiftsParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.subscription {
            query.push(("subscription", v.to_string()));
        }
        if let Some(v) = &params.subscription_length {
            query.push(("subscription_length", v.to_string()));
        }
        if let Some(v) = &params.subscription_period {
            query.push(("subscription_period", v.to_string()));
        }
        self.client.request(
            "get",
            "/gifts",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Hytale
    /// `GET /hytale`
    pub async fn category_hytale(
        &self,
        params: MarketCategoryHytaleParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.edition {
            for item in v {
                query.push(("edition[]", item.to_string()));
            }
        }
        if let Some(v) = &params.profiles_min {
            query.push(("profiles_min", v.to_string()));
        }
        if let Some(v) = &params.profiles_max {
            query.push(("profiles_max", v.to_string()));
        }
        self.client.request(
            "get",
            "/hytale",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Instagram
    /// `GET /instagram`
    pub async fn category_instagram(
        &self,
        params: MarketCategoryInstagramParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.cookies {
            query.push(("cookies", v.to_string()));
        }
        if let Some(v) = &params.login_without_cookies {
            query.push(("login_without_cookies", v.to_string()));
        }
        if let Some(v) = &params.followers_min {
            query.push(("followers_min", v.to_string()));
        }
        if let Some(v) = &params.followers_max {
            query.push(("followers_max", v.to_string()));
        }
        if let Some(v) = &params.post_min {
            query.push(("post_min", v.to_string()));
        }
        if let Some(v) = &params.post_max {
            query.push(("post_max", v.to_string()));
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        self.client.request(
            "get",
            "/instagram",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// miHoYo
    /// `GET /mihoyo`
    pub async fn category_mihoyo(
        &self,
        params: MarketCategoryMihoyoParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.email {
            query.push(("email", v.to_string()));
        }
        if let Some(v) = &params.ea {
            query.push(("ea", v.to_string()));
        }
        if let Some(v) = &params.region {
            for item in v {
                query.push(("region", item.to_string()));
            }
        }
        if let Some(v) = &params.not_region {
            for item in v {
                query.push(("not_region", item.to_string()));
            }
        }
        if let Some(v) = &params.genshin_character {
            for item in v {
                query.push(("genshin_character[]", item.to_string()));
            }
        }
        if let Some(v) = &params.genshin_character_constellations {
            query.push(("genshin_character_constellations", v.to_string()));
        }
        if let Some(v) = &params.genshin_character_constellations_max {
            query.push(("genshin_character_constellations_max", v.to_string()));
        }
        if let Some(v) = &params.genshin_weapon {
            for item in v {
                query.push(("genshin_weapon[]", item.to_string()));
            }
        }
        if let Some(v) = &params.genshin_char_min {
            query.push(("genshin_char_min", v.to_string()));
        }
        if let Some(v) = &params.genshin_char_max {
            query.push(("genshin_char_max", v.to_string()));
        }
        if let Some(v) = &params.genshin_legendary_min {
            query.push(("genshin_legendary_min", v.to_string()));
        }
        if let Some(v) = &params.genshin_legendary_max {
            query.push(("genshin_legendary_max", v.to_string()));
        }
        if let Some(v) = &params.genshin_level_min {
            query.push(("genshin_level_min", v.to_string()));
        }
        if let Some(v) = &params.genshin_level_max {
            query.push(("genshin_level_max", v.to_string()));
        }
        if let Some(v) = &params.genshin_legendary_weapon_min {
            query.push(("genshin_legendary_weapon_min", v.to_string()));
        }
        if let Some(v) = &params.genshin_legendary_weapon_max {
            query.push(("genshin_legendary_weapon_max", v.to_string()));
        }
        if let Some(v) = &params.constellations_min {
            query.push(("constellations_min", v.to_string()));
        }
        if let Some(v) = &params.constellations_max {
            query.push(("constellations_max", v.to_string()));
        }
        if let Some(v) = &params.genshin_achievement_min {
            query.push(("genshin_achievement_min", v.to_string()));
        }
        if let Some(v) = &params.genshin_achievement_max {
            query.push(("genshin_achievement_max", v.to_string()));
        }
        if let Some(v) = &params.genshin_currency_min {
            query.push(("genshin_currency_min", v.to_string()));
        }
        if let Some(v) = &params.genshin_currency_max {
            query.push(("genshin_currency_max", v.to_string()));
        }
        if let Some(v) = &params.honkai_character {
            for item in v {
                query.push(("honkai_character[]", item.to_string()));
            }
        }
        if let Some(v) = &params.honkai_character_eidolons {
            query.push(("honkai_character_eidolons", v.to_string()));
        }
        if let Some(v) = &params.honkai_character_eidolons_max {
            query.push(("honkai_character_eidolons_max", v.to_string()));
        }
        if let Some(v) = &params.honkai_weapon {
            for item in v {
                query.push(("honkai_weapon[]", item.to_string()));
            }
        }
        if let Some(v) = &params.honkai_char_min {
            query.push(("honkai_char_min", v.to_string()));
        }
        if let Some(v) = &params.honkai_char_max {
            query.push(("honkai_char_max", v.to_string()));
        }
        if let Some(v) = &params.honkai_legendary_min {
            query.push(("honkai_legendary_min", v.to_string()));
        }
        if let Some(v) = &params.honkai_legendary_max {
            query.push(("honkai_legendary_max", v.to_string()));
        }
        if let Some(v) = &params.honkai_level_min {
            query.push(("honkai_level_min", v.to_string()));
        }
        if let Some(v) = &params.honkai_level_max {
            query.push(("honkai_level_max", v.to_string()));
        }
        if let Some(v) = &params.honkai_legendary_weapon_min {
            query.push(("honkai_legendary_weapon_min", v.to_string()));
        }
        if let Some(v) = &params.honkai_legendary_weapon_max {
            query.push(("honkai_legendary_weapon_max", v.to_string()));
        }
        if let Some(v) = &params.eidolons_min {
            query.push(("eidolons_min", v.to_string()));
        }
        if let Some(v) = &params.eidolons_max {
            query.push(("eidolons_max", v.to_string()));
        }
        if let Some(v) = &params.honkai_achievement_min {
            query.push(("honkai_achievement_min", v.to_string()));
        }
        if let Some(v) = &params.honkai_achievement_max {
            query.push(("honkai_achievement_max", v.to_string()));
        }
        if let Some(v) = &params.honkai_currency_min {
            query.push(("honkai_currency_min", v.to_string()));
        }
        if let Some(v) = &params.honkai_currency_max {
            query.push(("honkai_currency_max", v.to_string()));
        }
        if let Some(v) = &params.zenless_character {
            for item in v {
                query.push(("zenless_character[]", item.to_string()));
            }
        }
        if let Some(v) = &params.zenless_character_cinemas {
            query.push(("zenless_character_cinemas", v.to_string()));
        }
        if let Some(v) = &params.zenless_character_cinemas_max {
            query.push(("zenless_character_cinemas_max", v.to_string()));
        }
        if let Some(v) = &params.zenless_weapon {
            for item in v {
                query.push(("zenless_weapon[]", item.to_string()));
            }
        }
        if let Some(v) = &params.zenless_legendary_min {
            query.push(("zenless_legendary_min", v.to_string()));
        }
        if let Some(v) = &params.zenless_legendary_max {
            query.push(("zenless_legendary_max", v.to_string()));
        }
        if let Some(v) = &params.cinemas_min {
            query.push(("cinemas_min", v.to_string()));
        }
        if let Some(v) = &params.cinemas_max {
            query.push(("cinemas_max", v.to_string()));
        }
        if let Some(v) = &params.zenless_legendary_weapon_min {
            query.push(("zenless_legendary_weapon_min", v.to_string()));
        }
        if let Some(v) = &params.zenless_legendary_weapon_max {
            query.push(("zenless_legendary_weapon_max", v.to_string()));
        }
        if let Some(v) = &params.zenless_char_min {
            query.push(("zenless_char_min", v.to_string()));
        }
        if let Some(v) = &params.zenless_char_max {
            query.push(("zenless_char_max", v.to_string()));
        }
        if let Some(v) = &params.zenless_level_min {
            query.push(("zenless_level_min", v.to_string()));
        }
        if let Some(v) = &params.zenless_level_max {
            query.push(("zenless_level_max", v.to_string()));
        }
        if let Some(v) = &params.zenless_achievement_min {
            query.push(("zenless_achievement_min", v.to_string()));
        }
        if let Some(v) = &params.zenless_achievement_max {
            query.push(("zenless_achievement_max", v.to_string()));
        }
        if let Some(v) = &params.zenless_currency_min {
            query.push(("zenless_currency_min", v.to_string()));
        }
        if let Some(v) = &params.zenless_currency_max {
            query.push(("zenless_currency_max", v.to_string()));
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        self.client.request(
            "get",
            "/mihoyo",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Minecraft
    /// `GET /minecraft`
    pub async fn category_minecraft(
        &self,
        params: MarketCategoryMinecraftParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.subscription {
            query.push(("subscription", v.to_string()));
        }
        if let Some(v) = &params.subscription_length {
            query.push(("subscription_length", v.to_string()));
        }
        if let Some(v) = &params.subscription_period {
            query.push(("subscription_period", v.to_string()));
        }
        if let Some(v) = &params.autorenewal {
            query.push(("autorenewal", v.to_string()));
        }
        if let Some(v) = &params.java {
            query.push(("java", v.to_string()));
        }
        if let Some(v) = &params.bedrock {
            query.push(("bedrock", v.to_string()));
        }
        if let Some(v) = &params.dungeons {
            query.push(("dungeons", v.to_string()));
        }
        if let Some(v) = &params.legends {
            query.push(("legends", v.to_string()));
        }
        if let Some(v) = &params.change_nickname {
            query.push(("change_nickname", v.to_string()));
        }
        if let Some(v) = &params.capes {
            for item in v {
                query.push(("capes[]", item.to_string()));
            }
        }
        if let Some(v) = &params.capes_min {
            query.push(("capes_min", v.to_string()));
        }
        if let Some(v) = &params.capes_max {
            query.push(("capes_max", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.hypixel_ban {
            query.push(("hypixel_ban", v.to_string()));
        }
        if let Some(v) = &params.hypixel_skyblock_api_enabled {
            query.push(("hypixel_skyblock_api_enabled", v.to_string()));
        }
        if let Some(v) = &params.rank_hypixel {
            for item in v {
                query.push(("rank_hypixel[]", item.to_string()));
            }
        }
        if let Some(v) = &params.level_hypixel_min {
            query.push(("level_hypixel_min", v.to_string()));
        }
        if let Some(v) = &params.level_hypixel_max {
            query.push(("level_hypixel_max", v.to_string()));
        }
        if let Some(v) = &params.achievement_hypixel_min {
            query.push(("achievement_hypixel_min", v.to_string()));
        }
        if let Some(v) = &params.achievement_hypixel_max {
            query.push(("achievement_hypixel_max", v.to_string()));
        }
        if let Some(v) = &params.level_hypixel_skyblock_min {
            query.push(("level_hypixel_skyblock_min", v.to_string()));
        }
        if let Some(v) = &params.level_hypixel_skyblock_max {
            query.push(("level_hypixel_skyblock_max", v.to_string()));
        }
        if let Some(v) = &params.net_worth_hypixel_skyblock_min {
            query.push(("net_worth_hypixel_skyblock_min", v.to_string()));
        }
        if let Some(v) = &params.net_worth_hypixel_skyblock_max {
            query.push(("net_worth_hypixel_skyblock_max", v.to_string()));
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        if let Some(v) = &params.last_login_hypixel {
            query.push(("last_login_hypixel", v.to_string()));
        }
        if let Some(v) = &params.last_login_hypixel_period {
            query.push(("last_login_hypixel_period", v.to_string()));
        }
        if let Some(v) = &params.can_change_details {
            query.push(("can_change_details", v.to_string()));
        }
        if let Some(v) = &params.nickname_length_min {
            query.push(("nickname_length_min", v.to_string()));
        }
        if let Some(v) = &params.nickname_length_max {
            query.push(("nickname_length_max", v.to_string()));
        }
        if let Some(v) = &params.hypixel_ban_parsed {
            query.push(("hypixel_ban_parsed", v.to_string()));
        }
        if let Some(v) = &params.minecoins_min {
            query.push(("minecoins_min", v.to_string()));
        }
        if let Some(v) = &params.minecoins_max {
            query.push(("minecoins_max", v.to_string()));
        }
        self.client.request(
            "get",
            "/minecraft",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Riot
    /// `GET /riot`
    pub async fn category_riot(
        &self,
        params: MarketCategoryRiotParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.rmin {
            query.push(("rmin", v.to_string()));
        }
        if let Some(v) = &params.rmax {
            query.push(("rmax", v.to_string()));
        }
        if let Some(v) = &params.last_rmin {
            query.push(("last_rmin", v.to_string()));
        }
        if let Some(v) = &params.last_rmax {
            query.push(("last_rmax", v.to_string()));
        }
        if let Some(v) = &params.previous_rmin {
            query.push(("previous_rmin", v.to_string()));
        }
        if let Some(v) = &params.previous_rmax {
            query.push(("previous_rmax", v.to_string()));
        }
        if let Some(v) = &params.weapon_skin {
            for item in v {
                query.push(("weaponSkin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.buddy {
            for item in v {
                query.push(("buddy[]", item.to_string()));
            }
        }
        if let Some(v) = &params.agent {
            for item in v {
                query.push(("agent[]", item.to_string()));
            }
        }
        if let Some(v) = &params.champion {
            for item in v {
                query.push(("champion[]", item.to_string()));
            }
        }
        if let Some(v) = &params.skin {
            for item in v {
                query.push(("skin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.valorant_level_min {
            query.push(("valorant_level_min", v.to_string()));
        }
        if let Some(v) = &params.valorant_level_max {
            query.push(("valorant_level_max", v.to_string()));
        }
        if let Some(v) = &params.lol_level_min {
            query.push(("lol_level_min", v.to_string()));
        }
        if let Some(v) = &params.lol_level_max {
            query.push(("lol_level_max", v.to_string()));
        }
        if let Some(v) = &params.inv_min {
            query.push(("inv_min", v.to_string()));
        }
        if let Some(v) = &params.inv_max {
            query.push(("inv_max", v.to_string()));
        }
        if let Some(v) = &params.vp_min {
            query.push(("vp_min", v.to_string()));
        }
        if let Some(v) = &params.vp_max {
            query.push(("vp_max", v.to_string()));
        }
        if let Some(v) = &params.valorant_smin {
            query.push(("valorant_smin", v.to_string()));
        }
        if let Some(v) = &params.valorant_smax {
            query.push(("valorant_smax", v.to_string()));
        }
        if let Some(v) = &params.valorant_rank_type {
            for item in v {
                query.push(("valorant_rank_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.amin {
            query.push(("amin", v.to_string()));
        }
        if let Some(v) = &params.amax {
            query.push(("amax", v.to_string()));
        }
        if let Some(v) = &params.valorant_region {
            for item in v {
                query.push(("valorant_region[]", item.to_string()));
            }
        }
        if let Some(v) = &params.valorant_not_region {
            for item in v {
                query.push(("valorant_not_region[]", item.to_string()));
            }
        }
        if let Some(v) = &params.lol_region {
            for item in v {
                query.push(("lol_region[]", item.to_string()));
            }
        }
        if let Some(v) = &params.lol_not_region {
            for item in v {
                query.push(("lol_not_region[]", item.to_string()));
            }
        }
        if let Some(v) = &params.knife {
            query.push(("knife", v.to_string()));
        }
        if let Some(v) = &params.lol_smin {
            query.push(("lol_smin", v.to_string()));
        }
        if let Some(v) = &params.lol_smax {
            query.push(("lol_smax", v.to_string()));
        }
        if let Some(v) = &params.champion_min {
            query.push(("champion_min", v.to_string()));
        }
        if let Some(v) = &params.champion_max {
            query.push(("champion_max", v.to_string()));
        }
        if let Some(v) = &params.win_rate_min {
            query.push(("win_rate_min", v.to_string()));
        }
        if let Some(v) = &params.win_rate_max {
            query.push(("win_rate_max", v.to_string()));
        }
        if let Some(v) = &params.blue_min {
            query.push(("blue_min", v.to_string()));
        }
        if let Some(v) = &params.blue_max {
            query.push(("blue_max", v.to_string()));
        }
        if let Some(v) = &params.orange_min {
            query.push(("orange_min", v.to_string()));
        }
        if let Some(v) = &params.orange_max {
            query.push(("orange_max", v.to_string()));
        }
        if let Some(v) = &params.mythic_min {
            query.push(("mythic_min", v.to_string()));
        }
        if let Some(v) = &params.mythic_max {
            query.push(("mythic_max", v.to_string()));
        }
        if let Some(v) = &params.riot_min {
            query.push(("riot_min", v.to_string()));
        }
        if let Some(v) = &params.riot_max {
            query.push(("riot_max", v.to_string()));
        }
        if let Some(v) = &params.email {
            query.push(("email", v.to_string()));
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.valorant_knife_min {
            query.push(("valorant_knife_min", v.to_string()));
        }
        if let Some(v) = &params.valorant_knife_max {
            query.push(("valorant_knife_max", v.to_string()));
        }
        if let Some(v) = &params.rp_min {
            query.push(("rp_min", v.to_string()));
        }
        if let Some(v) = &params.rp_max {
            query.push(("rp_max", v.to_string()));
        }
        if let Some(v) = &params.fa_min {
            query.push(("fa_min", v.to_string()));
        }
        if let Some(v) = &params.fa_max {
            query.push(("fa_max", v.to_string()));
        }
        if let Some(v) = &params.lol_rank {
            for item in v {
                query.push(("lol_rank[]", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/riot",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Roblox
    /// `GET /roblox`
    pub async fn category_roblox(
        &self,
        params: MarketCategoryRobloxParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email {
            query.push(("email", v.to_string()));
        }
        if let Some(v) = &params.robux_min {
            query.push(("robux_min", v.to_string()));
        }
        if let Some(v) = &params.robux_max {
            query.push(("robux_max", v.to_string()));
        }
        if let Some(v) = &params.friends_min {
            query.push(("friends_min", v.to_string()));
        }
        if let Some(v) = &params.friends_max {
            query.push(("friends_max", v.to_string()));
        }
        if let Some(v) = &params.followers_min {
            query.push(("followers_min", v.to_string()));
        }
        if let Some(v) = &params.followers_max {
            query.push(("followers_max", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country", item.to_string()));
            }
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        if let Some(v) = &params.subscription {
            query.push(("subscription", v.to_string()));
        }
        if let Some(v) = &params.subscription_length {
            query.push(("subscription_length", v.to_string()));
        }
        if let Some(v) = &params.subscription_period {
            query.push(("subscription_period", v.to_string()));
        }
        if let Some(v) = &params.autorenewal {
            query.push(("autorenewal", v.to_string()));
        }
        if let Some(v) = &params.xbox_connected {
            query.push(("xbox_connected", v.to_string()));
        }
        if let Some(v) = &params.psn_connected {
            query.push(("psn_connected", v.to_string()));
        }
        if let Some(v) = &params.verified {
            query.push(("verified", v.to_string()));
        }
        if let Some(v) = &params.age_verified {
            query.push(("age_verified", v.to_string()));
        }
        if let Some(v) = &params.incoming_robux_total_min {
            query.push(("incoming_robux_total_min", v.to_string()));
        }
        if let Some(v) = &params.incoming_robux_total_max {
            query.push(("incoming_robux_total_max", v.to_string()));
        }
        if let Some(v) = &params.limited_price_min {
            query.push(("limited_price_min", v.to_string()));
        }
        if let Some(v) = &params.limited_price_max {
            query.push(("limited_price_max", v.to_string()));
        }
        if let Some(v) = &params.gamepass_min {
            query.push(("gamepass_min", v.to_string()));
        }
        if let Some(v) = &params.gamepass_max {
            query.push(("gamepass_max", v.to_string()));
        }
        if let Some(v) = &params.game_donations {
            query.push(("game_donations", v.to_string()));
        }
        if let Some(v) = &params.inv_min {
            query.push(("inv_min", v.to_string()));
        }
        if let Some(v) = &params.inv_max {
            query.push(("inv_max", v.to_string()));
        }
        if let Some(v) = &params.ugc_limited_price_min {
            query.push(("ugc_limited_price_min", v.to_string()));
        }
        if let Some(v) = &params.ugc_limited_price_max {
            query.push(("ugc_limited_price_max", v.to_string()));
        }
        if let Some(v) = &params.credit_balance_min {
            query.push(("credit_balance_min", v.to_string()));
        }
        if let Some(v) = &params.credit_balance_max {
            query.push(("credit_balance_max", v.to_string()));
        }
        if let Some(v) = &params.offsale_min {
            query.push(("offsale_min", v.to_string()));
        }
        if let Some(v) = &params.offsale_max {
            query.push(("offsale_max", v.to_string()));
        }
        if let Some(v) = &params.voice {
            query.push(("voice", v.to_string()));
        }
        if let Some(v) = &params.age_group {
            for item in v {
                query.push(("age_group[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_age_group {
            for item in v {
                query.push(("not_age_group[]", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/roblox",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Social Club
    /// `GET /socialclub`
    pub async fn category_social_club(
        &self,
        params: MarketCategorySocialClubParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.level_min {
            query.push(("level_min", v.to_string()));
        }
        if let Some(v) = &params.level_max {
            query.push(("level_max", v.to_string()));
        }
        if let Some(v) = &params.cash_min {
            query.push(("cash_min", v.to_string()));
        }
        if let Some(v) = &params.cash_max {
            query.push(("cash_max", v.to_string()));
        }
        if let Some(v) = &params.bank_cash_min {
            query.push(("bank_cash_min", v.to_string()));
        }
        if let Some(v) = &params.bank_cash_max {
            query.push(("bank_cash_max", v.to_string()));
        }
        if let Some(v) = &params.game {
            for item in v {
                query.push(("game[]", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/socialclub",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Steam
    /// `GET /steam`
    pub async fn category_steam(
        &self,
        params: MarketCategorySteamParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.game {
            for item in v {
                query.push(("game[]", item.to_string()));
            }
        }
        if let Some(v) = &params.hours_played {
            query.push(("hours_played", v.to_string()));
        }
        if let Some(v) = &params.hours_played_max {
            query.push(("hours_played_max", v.to_string()));
        }
        if let Some(v) = &params.eg {
            query.push(("eg", v.to_string()));
        }
        if let Some(v) = &params.vac {
            for item in v {
                query.push(("vac[]", item.to_string()));
            }
        }
        if let Some(v) = &params.vac_skip_game_check {
            query.push(("vac_skip_game_check", v.to_string()));
        }
        if let Some(v) = &params.rt {
            query.push(("rt", v.to_string()));
        }
        if let Some(v) = &params.trade_ban {
            query.push(("trade_ban", v.to_string()));
        }
        if let Some(v) = &params.trade_limit {
            query.push(("trade_limit", v.to_string()));
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.limit {
            query.push(("limit", v.to_string()));
        }
        if let Some(v) = &params.mafile {
            query.push(("mafile", v.to_string()));
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        if let Some(v) = &params.lmin {
            query.push(("lmin", v.to_string()));
        }
        if let Some(v) = &params.lmax {
            query.push(("lmax", v.to_string()));
        }
        if let Some(v) = &params.rmin {
            query.push(("rmin", v.to_string()));
        }
        if let Some(v) = &params.rmax {
            query.push(("rmax", v.to_string()));
        }
        if let Some(v) = &params.wingman_rmin {
            query.push(("wingman_rmin", v.to_string()));
        }
        if let Some(v) = &params.wingman_rmax {
            query.push(("wingman_rmax", v.to_string()));
        }
        if let Some(v) = &params.no_vac {
            query.push(("no_vac", v.to_string()));
        }
        if let Some(v) = &params.mm_ban {
            query.push(("mm_ban", v.to_string()));
        }
        if let Some(v) = &params.balance_min {
            query.push(("balance_min", v.to_string()));
        }
        if let Some(v) = &params.balance_max {
            query.push(("balance_max", v.to_string()));
        }
        if let Some(v) = &params.inv_game {
            query.push(("inv_game", v.to_string()));
        }
        if let Some(v) = &params.inv_min {
            query.push(("inv_min", v.to_string()));
        }
        if let Some(v) = &params.inv_max {
            query.push(("inv_max", v.to_string()));
        }
        if let Some(v) = &params.friends_min {
            query.push(("friends_min", v.to_string()));
        }
        if let Some(v) = &params.friends_max {
            query.push(("friends_max", v.to_string()));
        }
        if let Some(v) = &params.gmin {
            query.push(("gmin", v.to_string()));
        }
        if let Some(v) = &params.gmax {
            query.push(("gmax", v.to_string()));
        }
        if let Some(v) = &params.win_count_min {
            query.push(("win_count_min", v.to_string()));
        }
        if let Some(v) = &params.win_count_max {
            query.push(("win_count_max", v.to_string()));
        }
        if let Some(v) = &params.medal_id {
            for item in v {
                query.push(("medal_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.medal_operator_or {
            query.push(("medal_operator_or", v.to_string()));
        }
        if let Some(v) = &params.medal_min {
            query.push(("medal_min", v.to_string()));
        }
        if let Some(v) = &params.medal_max {
            query.push(("medal_max", v.to_string()));
        }
        if let Some(v) = &params.gift {
            for item in v {
                query.push(("gift[]", item.to_string()));
            }
        }
        if let Some(v) = &params.gift_min {
            query.push(("gift_min", v.to_string()));
        }
        if let Some(v) = &params.gift_max {
            query.push(("gift_max", v.to_string()));
        }
        if let Some(v) = &params.recently_hours_min {
            query.push(("recently_hours_min", v.to_string()));
        }
        if let Some(v) = &params.recently_hours_max {
            query.push(("recently_hours_max", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.cs2_profile_rank_min {
            query.push(("cs2_profile_rank_min", v.to_string()));
        }
        if let Some(v) = &params.cs2_profile_rank_max {
            query.push(("cs2_profile_rank_max", v.to_string()));
        }
        if let Some(v) = &params.solommr_min {
            query.push(("solommr_min", v.to_string()));
        }
        if let Some(v) = &params.solommr_max {
            query.push(("solommr_max", v.to_string()));
        }
        if let Some(v) = &params.d2_game_count_min {
            query.push(("d2_game_count_min", v.to_string()));
        }
        if let Some(v) = &params.d2_game_count_max {
            query.push(("d2_game_count_max", v.to_string()));
        }
        if let Some(v) = &params.d2_win_count_min {
            query.push(("d2_win_count_min", v.to_string()));
        }
        if let Some(v) = &params.d2_win_count_max {
            query.push(("d2_win_count_max", v.to_string()));
        }
        if let Some(v) = &params.d2_behavior_min {
            query.push(("d2_behavior_min", v.to_string()));
        }
        if let Some(v) = &params.d2_behavior_max {
            query.push(("d2_behavior_max", v.to_string()));
        }
        if let Some(v) = &params.faceit_lvl_min {
            query.push(("faceit_lvl_min", v.to_string()));
        }
        if let Some(v) = &params.faceit_lvl_max {
            query.push(("faceit_lvl_max", v.to_string()));
        }
        if let Some(v) = &params.points_min {
            query.push(("points_min", v.to_string()));
        }
        if let Some(v) = &params.points_max {
            query.push(("points_max", v.to_string()));
        }
        if let Some(v) = &params.relevant_gmin {
            query.push(("relevant_gmin", v.to_string()));
        }
        if let Some(v) = &params.relevant_gmax {
            query.push(("relevant_gmax", v.to_string()));
        }
        if let Some(v) = &params.last_trans_date {
            query.push(("last_trans_date", v.to_string()));
        }
        if let Some(v) = &params.last_trans_date_period {
            query.push(("last_trans_date_period", v.to_string()));
        }
        if let Some(v) = &params.last_trans_date_later {
            query.push(("last_trans_date_later", v.to_string()));
        }
        if let Some(v) = &params.last_trans_date_period_later {
            query.push(("last_trans_date_period_later", v.to_string()));
        }
        if let Some(v) = &params.no_trans {
            query.push(("no_trans", v.to_string()));
        }
        if let Some(v) = &params.trans {
            query.push(("trans", v.to_string()));
        }
        if let Some(v) = &params.gifts_purchase_min {
            query.push(("gifts_purchase_min", v.to_string()));
        }
        if let Some(v) = &params.gifts_purchase_max {
            query.push(("gifts_purchase_max", v.to_string()));
        }
        if let Some(v) = &params.refunds_purchase_min {
            query.push(("refunds_purchase_min", v.to_string()));
        }
        if let Some(v) = &params.refunds_purchase_max {
            query.push(("refunds_purchase_max", v.to_string()));
        }
        if let Some(v) = &params.ingame_purchase_min {
            query.push(("ingame_purchase_min", v.to_string()));
        }
        if let Some(v) = &params.ingame_purchase_max {
            query.push(("ingame_purchase_max", v.to_string()));
        }
        if let Some(v) = &params.games_purchase_min {
            query.push(("games_purchase_min", v.to_string()));
        }
        if let Some(v) = &params.games_purchase_max {
            query.push(("games_purchase_max", v.to_string()));
        }
        if let Some(v) = &params.purchase_min {
            query.push(("purchase_min", v.to_string()));
        }
        if let Some(v) = &params.purchase_max {
            query.push(("purchase_max", v.to_string()));
        }
        if let Some(v) = &params.has_activated_keys {
            query.push(("has_activated_keys", v.to_string()));
        }
        if let Some(v) = &params.elo_min {
            query.push(("elo_min", v.to_string()));
        }
        if let Some(v) = &params.elo_max {
            query.push(("elo_max", v.to_string()));
        }
        if let Some(v) = &params.cs2_map_rank {
            query.push(("cs2_map_rank", v.to_string()));
        }
        if let Some(v) = &params.cs2_map_rmin {
            query.push(("cs2_map_rmin", v.to_string()));
        }
        if let Some(v) = &params.cs2_map_rmax {
            query.push(("cs2_map_rmax", v.to_string()));
        }
        if let Some(v) = &params.has_faceit {
            query.push(("has_faceit", v.to_string()));
        }
        if let Some(v) = &params.faceit_csgo_lvl_min {
            query.push(("faceit_csgo_lvl_min", v.to_string()));
        }
        if let Some(v) = &params.faceit_csgo_lvl_max {
            query.push(("faceit_csgo_lvl_max", v.to_string()));
        }
        if let Some(v) = &params.rust_deaths_min {
            query.push(("rust_deaths_min", v.to_string()));
        }
        if let Some(v) = &params.rust_deaths_max {
            query.push(("rust_deaths_max", v.to_string()));
        }
        if let Some(v) = &params.rust_kills_min {
            query.push(("rust_kills_min", v.to_string()));
        }
        if let Some(v) = &params.rust_kills_max {
            query.push(("rust_kills_max", v.to_string()));
        }
        if let Some(v) = &params.d2_last_match_date {
            query.push(("d2_last_match_date", v.to_string()));
        }
        if let Some(v) = &params.d2_last_match_date_period {
            query.push(("d2_last_match_date_period", v.to_string()));
        }
        if let Some(v) = &params.cards_min {
            query.push(("cards_min", v.to_string()));
        }
        if let Some(v) = &params.cards_max {
            query.push(("cards_max", v.to_string()));
        }
        if let Some(v) = &params.cards_games_min {
            query.push(("cards_games_min", v.to_string()));
        }
        if let Some(v) = &params.cards_games_max {
            query.push(("cards_games_max", v.to_string()));
        }
        if let Some(v) = &params.skip_vac_inv {
            query.push(("skip_vac_inv", v.to_string()));
        }
        self.client.request(
            "get",
            "/steam",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Supercell
    /// `GET /supercell`
    pub async fn category_supercell(
        &self,
        params: MarketCategorySupercellParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.eg {
            query.push(("eg", v.to_string()));
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.brawl_level_min {
            query.push(("brawl_level_min", v.to_string()));
        }
        if let Some(v) = &params.brawl_level_max {
            query.push(("brawl_level_max", v.to_string()));
        }
        if let Some(v) = &params.brawl_cup_min {
            query.push(("brawl_cup_min", v.to_string()));
        }
        if let Some(v) = &params.brawl_cup_max {
            query.push(("brawl_cup_max", v.to_string()));
        }
        if let Some(v) = &params.brawl_wins_min {
            query.push(("brawl_wins_min", v.to_string()));
        }
        if let Some(v) = &params.brawl_wins_max {
            query.push(("brawl_wins_max", v.to_string()));
        }
        if let Some(v) = &params.brawl_pass {
            query.push(("brawl_pass", v.to_string()));
        }
        if let Some(v) = &params.brawler {
            for item in v {
                query.push(("brawler[]", item.to_string()));
            }
        }
        if let Some(v) = &params.brawlers_min {
            query.push(("brawlers_min", v.to_string()));
        }
        if let Some(v) = &params.brawlers_max {
            query.push(("brawlers_max", v.to_string()));
        }
        if let Some(v) = &params.legendary_brawlers_min {
            query.push(("legendary_brawlers_min", v.to_string()));
        }
        if let Some(v) = &params.legendary_brawlers_max {
            query.push(("legendary_brawlers_max", v.to_string()));
        }
        if let Some(v) = &params.royale_level_min {
            query.push(("royale_level_min", v.to_string()));
        }
        if let Some(v) = &params.royale_level_max {
            query.push(("royale_level_max", v.to_string()));
        }
        if let Some(v) = &params.royale_cup_min {
            query.push(("royale_cup_min", v.to_string()));
        }
        if let Some(v) = &params.royale_cup_max {
            query.push(("royale_cup_max", v.to_string()));
        }
        if let Some(v) = &params.royale_wins_min {
            query.push(("royale_wins_min", v.to_string()));
        }
        if let Some(v) = &params.royale_wins_max {
            query.push(("royale_wins_max", v.to_string()));
        }
        if let Some(v) = &params.king_level_min {
            query.push(("king_level_min", v.to_string()));
        }
        if let Some(v) = &params.king_level_max {
            query.push(("king_level_max", v.to_string()));
        }
        if let Some(v) = &params.royale_pass {
            query.push(("royale_pass", v.to_string()));
        }
        if let Some(v) = &params.clash_level_min {
            query.push(("clash_level_min", v.to_string()));
        }
        if let Some(v) = &params.clash_level_max {
            query.push(("clash_level_max", v.to_string()));
        }
        if let Some(v) = &params.clash_cup_min {
            query.push(("clash_cup_min", v.to_string()));
        }
        if let Some(v) = &params.clash_cup_max {
            query.push(("clash_cup_max", v.to_string()));
        }
        if let Some(v) = &params.clash_wins_min {
            query.push(("clash_wins_min", v.to_string()));
        }
        if let Some(v) = &params.clash_wins_max {
            query.push(("clash_wins_max", v.to_string()));
        }
        if let Some(v) = &params.clash_pass {
            query.push(("clash_pass", v.to_string()));
        }
        if let Some(v) = &params.total_heroes_level_min {
            query.push(("total_heroes_level_min", v.to_string()));
        }
        if let Some(v) = &params.total_heroes_level_max {
            query.push(("total_heroes_level_max", v.to_string()));
        }
        if let Some(v) = &params.total_troops_level_min {
            query.push(("total_troops_level_min", v.to_string()));
        }
        if let Some(v) = &params.total_troops_level_max {
            query.push(("total_troops_level_max", v.to_string()));
        }
        if let Some(v) = &params.total_spells_level_min {
            query.push(("total_spells_level_min", v.to_string()));
        }
        if let Some(v) = &params.total_spells_level_max {
            query.push(("total_spells_level_max", v.to_string()));
        }
        if let Some(v) = &params.total_builder_heroes_level_min {
            query.push(("total_builder_heroes_level_min", v.to_string()));
        }
        if let Some(v) = &params.total_builder_heroes_level_max {
            query.push(("total_builder_heroes_level_max", v.to_string()));
        }
        if let Some(v) = &params.total_builder_troops_level_min {
            query.push(("total_builder_troops_level_min", v.to_string()));
        }
        if let Some(v) = &params.total_builder_troops_level_max {
            query.push(("total_builder_troops_level_max", v.to_string()));
        }
        if let Some(v) = &params.town_hall_level_min {
            query.push(("town_hall_level_min", v.to_string()));
        }
        if let Some(v) = &params.town_hall_level_max {
            query.push(("town_hall_level_max", v.to_string()));
        }
        if let Some(v) = &params.builder_hall_level_min {
            query.push(("builder_hall_level_min", v.to_string()));
        }
        if let Some(v) = &params.builder_hall_level_max {
            query.push(("builder_hall_level_max", v.to_string()));
        }
        if let Some(v) = &params.builder_hall_cup_min {
            query.push(("builder_hall_cup_min", v.to_string()));
        }
        if let Some(v) = &params.builder_hall_cup_max {
            query.push(("builder_hall_cup_max", v.to_string()));
        }
        if let Some(v) = &params.creation_year_min {
            query.push(("creation_year_min", v.to_string()));
        }
        if let Some(v) = &params.creation_year_max {
            query.push(("creation_year_max", v.to_string()));
        }
        self.client.request(
            "get",
            "/supercell",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Telegram
    /// `GET /telegram`
    pub async fn category_telegram(
        &self,
        params: MarketCategoryTelegramParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.spam {
            query.push(("spam", v.to_string()));
        }
        if let Some(v) = &params.password {
            query.push(("password", v.to_string()));
        }
        if let Some(v) = &params.premium {
            query.push(("premium", v.to_string()));
        }
        if let Some(v) = &params.premium_expiration {
            query.push(("premium_expiration", v.to_string()));
        }
        if let Some(v) = &params.premium_expiration_period {
            query.push(("premium_expiration_period", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.min_channels {
            query.push(("min_channels", v.to_string()));
        }
        if let Some(v) = &params.max_channels {
            query.push(("max_channels", v.to_string()));
        }
        if let Some(v) = &params.min_chats {
            query.push(("min_chats", v.to_string()));
        }
        if let Some(v) = &params.max_chats {
            query.push(("max_chats", v.to_string()));
        }
        if let Some(v) = &params.min_conversations {
            query.push(("min_conversations", v.to_string()));
        }
        if let Some(v) = &params.max_conversations {
            query.push(("max_conversations", v.to_string()));
        }
        if let Some(v) = &params.min_admin {
            query.push(("min_admin", v.to_string()));
        }
        if let Some(v) = &params.max_admin {
            query.push(("max_admin", v.to_string()));
        }
        if let Some(v) = &params.min_admin_sub {
            query.push(("min_admin_sub", v.to_string()));
        }
        if let Some(v) = &params.max_admin_sub {
            query.push(("max_admin_sub", v.to_string()));
        }
        if let Some(v) = &params.dig_min {
            query.push(("dig_min", v.to_string()));
        }
        if let Some(v) = &params.dig_max {
            query.push(("dig_max", v.to_string()));
        }
        if let Some(v) = &params.min_contacts {
            query.push(("min_contacts", v.to_string()));
        }
        if let Some(v) = &params.max_contacts {
            query.push(("max_contacts", v.to_string()));
        }
        if let Some(v) = &params.min_stars {
            query.push(("min_stars", v.to_string()));
        }
        if let Some(v) = &params.max_stars {
            query.push(("max_stars", v.to_string()));
        }
        if let Some(v) = &params.birthday {
            query.push(("birthday", v.to_string()));
        }
        if let Some(v) = &params.birthday_period {
            query.push(("birthday_period", v.to_string()));
        }
        if let Some(v) = &params.birthday_after {
            query.push(("birthday_after", v.to_string()));
        }
        if let Some(v) = &params.birthday_after_period {
            query.push(("birthday_after_period", v.to_string()));
        }
        if let Some(v) = &params.min_id {
            query.push(("min_id", v.to_string()));
        }
        if let Some(v) = &params.max_id {
            query.push(("max_id", v.to_string()));
        }
        if let Some(v) = &params.allow_geo_spamblock {
            query.push(("allow_geo_spamblock", v.to_string()));
        }
        if let Some(v) = &params.min_gifts {
            query.push(("min_gifts", v.to_string()));
        }
        if let Some(v) = &params.max_gifts {
            query.push(("max_gifts", v.to_string()));
        }
        if let Some(v) = &params.min_nft_gifts {
            query.push(("min_nft_gifts", v.to_string()));
        }
        if let Some(v) = &params.max_nft_gifts {
            query.push(("max_nft_gifts", v.to_string()));
        }
        if let Some(v) = &params.min_gifts_stars {
            query.push(("min_gifts_stars", v.to_string()));
        }
        if let Some(v) = &params.max_gifts_stars {
            query.push(("max_gifts_stars", v.to_string()));
        }
        if let Some(v) = &params.min_gifts_convert_stars {
            query.push(("min_gifts_convert_stars", v.to_string()));
        }
        if let Some(v) = &params.max_gifts_convert_stars {
            query.push(("max_gifts_convert_stars", v.to_string()));
        }
        if let Some(v) = &params.dc_id {
            for item in v {
                query.push(("dc_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_dc_id {
            for item in v {
                query.push(("not_dc_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.email {
            query.push(("email", v.to_string()));
        }
        if let Some(v) = &params.min_bots {
            query.push(("min_bots", v.to_string()));
        }
        if let Some(v) = &params.max_bots {
            query.push(("max_bots", v.to_string()));
        }
        if let Some(v) = &params.min_bot_active_users {
            query.push(("min_bot_active_users", v.to_string()));
        }
        if let Some(v) = &params.max_bot_active_users {
            query.push(("max_bot_active_users", v.to_string()));
        }
        self.client.request(
            "get",
            "/telegram",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// TikTok
    /// `GET /tiktok`
    pub async fn category_tik_tok(
        &self,
        params: MarketCategoryTikTokParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        if let Some(v) = &params.followers_min {
            query.push(("followers_min", v.to_string()));
        }
        if let Some(v) = &params.followers_max {
            query.push(("followers_max", v.to_string()));
        }
        if let Some(v) = &params.post_min {
            query.push(("post_min", v.to_string()));
        }
        if let Some(v) = &params.post_max {
            query.push(("post_max", v.to_string()));
        }
        if let Some(v) = &params.like_min {
            query.push(("like_min", v.to_string()));
        }
        if let Some(v) = &params.like_max {
            query.push(("like_max", v.to_string()));
        }
        if let Some(v) = &params.coins_min {
            query.push(("coins_min", v.to_string()));
        }
        if let Some(v) = &params.coins_max {
            query.push(("coins_max", v.to_string()));
        }
        if let Some(v) = &params.cookie_login {
            query.push(("cookie_login", v.to_string()));
        }
        if let Some(v) = &params.verified {
            query.push(("verified", v.to_string()));
        }
        if let Some(v) = &params.email {
            query.push(("email", v.to_string()));
        }
        self.client.request(
            "get",
            "/tiktok",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Uplay
    /// `GET /uplay`
    pub async fn category_uplay(
        &self,
        params: MarketCategoryUplayParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.game {
            for item in v {
                query.push(("game[]", item.to_string()));
            }
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.gmin {
            query.push(("gmin", v.to_string()));
        }
        if let Some(v) = &params.gmax {
            query.push(("gmax", v.to_string()));
        }
        if let Some(v) = &params.subscription {
            query.push(("subscription", v.to_string()));
        }
        if let Some(v) = &params.subscription_length {
            query.push(("subscription_length", v.to_string()));
        }
        if let Some(v) = &params.subscription_period {
            query.push(("subscription_period", v.to_string()));
        }
        if let Some(v) = &params.r6_level_min {
            query.push(("r6_level_min", v.to_string()));
        }
        if let Some(v) = &params.r6_level_max {
            query.push(("r6_level_max", v.to_string()));
        }
        if let Some(v) = &params.r6_rank_min {
            query.push(("r6_rank_min", v.to_string()));
        }
        if let Some(v) = &params.r6_rank_max {
            query.push(("r6_rank_max", v.to_string()));
        }
        if let Some(v) = &params.r6_operators_min {
            query.push(("r6_operators_min", v.to_string()));
        }
        if let Some(v) = &params.r6_operators_max {
            query.push(("r6_operators_max", v.to_string()));
        }
        if let Some(v) = &params.r6_ban {
            query.push(("r6_ban", v.to_string()));
        }
        if let Some(v) = &params.r6_smin {
            query.push(("r6_smin", v.to_string()));
        }
        if let Some(v) = &params.r6_smax {
            query.push(("r6_smax", v.to_string()));
        }
        if let Some(v) = &params.r6_skin {
            for item in v {
                query.push(("r6_skin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.r6_operator {
            for item in v {
                query.push(("r6_operator[]", item.to_string()));
            }
        }
        if let Some(v) = &params.xbox_connected {
            query.push(("xbox_connected", v.to_string()));
        }
        if let Some(v) = &params.psn_connected {
            query.push(("psn_connected", v.to_string()));
        }
        if let Some(v) = &params.steam_connected {
            query.push(("steam_connected", v.to_string()));
        }
        if let Some(v) = &params.balance_min {
            query.push(("balance_min", v.to_string()));
        }
        if let Some(v) = &params.balance_max {
            query.push(("balance_max", v.to_string()));
        }
        if let Some(v) = &params.transactions {
            query.push(("transactions", v.to_string()));
        }
        if let Some(v) = &params.reg {
            query.push(("reg", v.to_string()));
        }
        if let Some(v) = &params.reg_period {
            query.push(("reg_period", v.to_string()));
        }
        self.client.request(
            "get",
            "/uplay",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// VPN
    /// `GET /vpn`
    pub async fn category_vpn(
        &self,
        params: MarketCategoryVpnParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.service {
            for item in v {
                query.push(("service[]", item.to_string()));
            }
        }
        if let Some(v) = &params.subscription_length {
            query.push(("subscription_length", v.to_string()));
        }
        if let Some(v) = &params.subscription_period {
            query.push(("subscription_period", v.to_string()));
        }
        if let Some(v) = &params.autorenewal {
            query.push(("autorenewal", v.to_string()));
        }
        self.client.request(
            "get",
            "/vpn",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Warface
    /// `GET /warface`
    pub async fn category_warface(
        &self,
        params: MarketCategoryWarfaceParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.rank_min {
            query.push(("rank_min", v.to_string()));
        }
        if let Some(v) = &params.rank_max {
            query.push(("rank_max", v.to_string()));
        }
        if let Some(v) = &params.bonus_rank_min {
            query.push(("bonus_rank_min", v.to_string()));
        }
        if let Some(v) = &params.bonus_rank_max {
            query.push(("bonus_rank_max", v.to_string()));
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.kredits_min {
            query.push(("kredits_min", v.to_string()));
        }
        if let Some(v) = &params.kredits_max {
            query.push(("kredits_max", v.to_string()));
        }
        if let Some(v) = &params.total_kredits_min {
            query.push(("total_kredits_min", v.to_string()));
        }
        if let Some(v) = &params.total_kredits_max {
            query.push(("total_kredits_max", v.to_string()));
        }
        self.client.request(
            "get",
            "/warface",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// World of Tanks
    /// `GET /world-of-tanks`
    pub async fn category_wot(
        &self,
        params: MarketCategoryWotParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.battles_min {
            query.push(("battles_min", v.to_string()));
        }
        if let Some(v) = &params.battles_max {
            query.push(("battles_max", v.to_string()));
        }
        if let Some(v) = &params.gold_min {
            query.push(("gold_min", v.to_string()));
        }
        if let Some(v) = &params.gold_max {
            query.push(("gold_max", v.to_string()));
        }
        if let Some(v) = &params.silver_min {
            query.push(("silver_min", v.to_string()));
        }
        if let Some(v) = &params.silver_max {
            query.push(("silver_max", v.to_string()));
        }
        if let Some(v) = &params.top_min {
            query.push(("top_min", v.to_string()));
        }
        if let Some(v) = &params.top_max {
            query.push(("top_max", v.to_string()));
        }
        if let Some(v) = &params.prem_min {
            query.push(("prem_min", v.to_string()));
        }
        if let Some(v) = &params.prem_max {
            query.push(("prem_max", v.to_string()));
        }
        if let Some(v) = &params.top_prem_min {
            query.push(("top_prem_min", v.to_string()));
        }
        if let Some(v) = &params.top_prem_max {
            query.push(("top_prem_max", v.to_string()));
        }
        if let Some(v) = &params.win_pmin {
            query.push(("win_pmin", v.to_string()));
        }
        if let Some(v) = &params.win_pmax {
            query.push(("win_pmax", v.to_string()));
        }
        if let Some(v) = &params.tank {
            for item in v {
                query.push(("tank[]", item.to_string()));
            }
        }
        if let Some(v) = &params.region {
            for item in v {
                query.push(("region[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_region {
            for item in v {
                query.push(("not_region[]", item.to_string()));
            }
        }
        if let Some(v) = &params.premium {
            query.push(("premium", v.to_string()));
        }
        if let Some(v) = &params.premium_expiration {
            query.push(("premium_expiration", v.to_string()));
        }
        if let Some(v) = &params.premium_expiration_period {
            query.push(("premium_expiration_period", v.to_string()));
        }
        if let Some(v) = &params.clan {
            query.push(("clan", v.to_string()));
        }
        if let Some(v) = &params.clan_role {
            for item in v {
                query.push(("clan_role[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_clan_role {
            for item in v {
                query.push(("not_clan_role[]", item.to_string()));
            }
        }
        if let Some(v) = &params.clan_gold_min {
            query.push(("clan_gold_min", v.to_string()));
        }
        if let Some(v) = &params.clan_gold_max {
            query.push(("clan_gold_max", v.to_string()));
        }
        if let Some(v) = &params.clan_credits_min {
            query.push(("clan_credits_min", v.to_string()));
        }
        if let Some(v) = &params.clan_credits_max {
            query.push(("clan_credits_max", v.to_string()));
        }
        if let Some(v) = &params.clan_crystal_min {
            query.push(("clan_crystal_min", v.to_string()));
        }
        if let Some(v) = &params.clan_crystal_max {
            query.push(("clan_crystal_max", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/world-of-tanks",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// WoT Blitz
    /// `GET /wot-blitz`
    pub async fn category_wot_blitz(
        &self,
        params: MarketCategoryWotBlitzParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.order_by {
            query.push(("order_by", v.to_string()));
        }
        if let Some(v) = &params.tag_id {
            for item in v {
                query.push(("tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_tag_id {
            for item in v {
                query.push(("not_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.public_tag_id {
            for item in v {
                query.push(("public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_public_tag_id {
            for item in v {
                query.push(("not_public_tag_id[]", item.to_string()));
            }
        }
        if let Some(v) = &params.origin {
            for item in v {
                query.push(("origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_origin {
            for item in v {
                query.push(("not_origin[]", item.to_string()));
            }
        }
        if let Some(v) = &params.user_id {
            query.push(("user_id", v.to_string()));
        }
        if let Some(v) = &params.nsb {
            query.push(("nsb", v.to_string()));
        }
        if let Some(v) = &params.sb {
            query.push(("sb", v.to_string()));
        }
        if let Some(v) = &params.nsb_by_me {
            query.push(("nsb_by_me", v.to_string()));
        }
        if let Some(v) = &params.sb_by_me {
            query.push(("sb_by_me", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.email_login_data {
            query.push(("email_login_data", v.to_string()));
        }
        if let Some(v) = &params.email_provider {
            for item in v {
                query.push(("email_provider[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_email_provider {
            query.push(("not_email_provider[]", v.to_string()));
        }
        if let Some(v) = &params.parse_same_item_ids {
            query.push(("parse_same_item_ids", v.to_string()));
        }
        if let Some(v) = &params.email_type {
            for item in v {
                query.push(("email_type[]", item.to_string()));
            }
        }
        if let Some(v) = &params.item_domain {
            query.push(("item_domain", v.to_string()));
        }
        if let Some(v) = &params.tel {
            query.push(("tel", v.to_string()));
        }
        if let Some(v) = &params.daybreak {
            query.push(("daybreak", v.to_string()));
        }
        if let Some(v) = &params.battles_min {
            query.push(("battles_min", v.to_string()));
        }
        if let Some(v) = &params.battles_max {
            query.push(("battles_max", v.to_string()));
        }
        if let Some(v) = &params.gold_min {
            query.push(("gold_min", v.to_string()));
        }
        if let Some(v) = &params.gold_max {
            query.push(("gold_max", v.to_string()));
        }
        if let Some(v) = &params.silver_min {
            query.push(("silver_min", v.to_string()));
        }
        if let Some(v) = &params.silver_max {
            query.push(("silver_max", v.to_string()));
        }
        if let Some(v) = &params.top_min {
            query.push(("top_min", v.to_string()));
        }
        if let Some(v) = &params.top_max {
            query.push(("top_max", v.to_string()));
        }
        if let Some(v) = &params.prem_min {
            query.push(("prem_min", v.to_string()));
        }
        if let Some(v) = &params.prem_max {
            query.push(("prem_max", v.to_string()));
        }
        if let Some(v) = &params.top_prem_min {
            query.push(("top_prem_min", v.to_string()));
        }
        if let Some(v) = &params.top_prem_max {
            query.push(("top_prem_max", v.to_string()));
        }
        if let Some(v) = &params.win_pmin {
            query.push(("win_pmin", v.to_string()));
        }
        if let Some(v) = &params.win_pmax {
            query.push(("win_pmax", v.to_string()));
        }
        if let Some(v) = &params.tank {
            for item in v {
                query.push(("tank[]", item.to_string()));
            }
        }
        if let Some(v) = &params.region {
            for item in v {
                query.push(("region[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_region {
            for item in v {
                query.push(("not_region[]", item.to_string()));
            }
        }
        if let Some(v) = &params.premium {
            query.push(("premium", v.to_string()));
        }
        if let Some(v) = &params.premium_expiration {
            query.push(("premium_expiration", v.to_string()));
        }
        if let Some(v) = &params.premium_expiration_period {
            query.push(("premium_expiration_period", v.to_string()));
        }
        if let Some(v) = &params.clan {
            query.push(("clan", v.to_string()));
        }
        if let Some(v) = &params.clan_role {
            for item in v {
                query.push(("clan_role[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_clan_role {
            for item in v {
                query.push(("not_clan_role[]", item.to_string()));
            }
        }
        if let Some(v) = &params.clan_gold_min {
            query.push(("clan_gold_min", v.to_string()));
        }
        if let Some(v) = &params.clan_gold_max {
            query.push(("clan_gold_max", v.to_string()));
        }
        if let Some(v) = &params.clan_credits_min {
            query.push(("clan_credits_min", v.to_string()));
        }
        if let Some(v) = &params.clan_credits_max {
            query.push(("clan_credits_max", v.to_string()));
        }
        if let Some(v) = &params.clan_crystal_min {
            query.push(("clan_crystal_min", v.to_string()));
        }
        if let Some(v) = &params.clan_crystal_max {
            query.push(("clan_crystal_max", v.to_string()));
        }
        if let Some(v) = &params.country {
            for item in v {
                query.push(("country[]", item.to_string()));
            }
        }
        if let Some(v) = &params.not_country {
            for item in v {
                query.push(("not_country[]", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/wot-blitz",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Custom Discounts ──

    /// Create Custom Discount
    /// `POST /custom-discounts`
    pub async fn custom_discounts_create(
        &self,
        params: MarketCustomDiscountsCreateParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("category_id".into(), serde_json::to_value(&params.category_id).unwrap_or_default());
        if let Some(v) = &params.currency {
            body.insert("currency".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("discount_percent".into(), serde_json::to_value(&params.discount_percent).unwrap_or_default());
        if let Some(v) = &params.max_price {
            body.insert("max_price".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("min_price".into(), serde_json::to_value(&params.min_price).unwrap_or_default());
        body.insert("user_id".into(), serde_json::to_value(&params.user_id).unwrap_or_default());
        self.client.request(
            "post",
            "/custom-discounts",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Custom Discount
    /// `DELETE /custom-discounts`
    pub async fn custom_discounts_delete(
        &self,
        discount_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/custom-discounts",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Custom Discount
    /// `PUT /custom-discounts`
    pub async fn custom_discounts_edit(
        &self,
        params: MarketCustomDiscountsEditParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("discount_id".into(), serde_json::to_value(&params.discount_id).unwrap_or_default());
        if let Some(v) = &params.discount_percent {
            body.insert("discount_percent".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.max_price {
            body.insert("max_price".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.min_price {
            body.insert("min_price".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "put",
            "/custom-discounts",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Custom Discounts
    /// `GET /custom-discounts`
    pub async fn custom_discounts_get(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/custom-discounts",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── IMAP ──

    /// Create IMAP Configuration
    /// `POST /imap`
    pub async fn imap_create(
        &self,
        params: MarketImapCreateParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("domain".into(), serde_json::to_value(&params.domain).unwrap_or_default());
        body.insert("imap_server".into(), serde_json::to_value(&params.imap_server).unwrap_or_default());
        body.insert("port".into(), serde_json::to_value(&params.port).unwrap_or_default());
        body.insert("secure".into(), serde_json::to_value(&params.secure).unwrap_or_default());
        self.client.request(
            "post",
            "/imap",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete IMAP Configuration
    /// `DELETE /imap`
    pub async fn imap_delete(
        &self,
        domain: String,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/imap",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Invoices ──

    /// Create Invoice
    /// `POST /invoice`
    pub async fn payments_invoice_create(
        &self,
        params: MarketPaymentsInvoiceCreateParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.additional_data {
            body.insert("additional_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("amount".into(), serde_json::to_value(&params.amount).unwrap_or_default());
        body.insert("comment".into(), serde_json::to_value(&params.comment).unwrap_or_default());
        body.insert("currency".into(), serde_json::to_value(&params.currency).unwrap_or_default());
        if let Some(v) = &params.is_test {
            body.insert("is_test".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.lifetime {
            body.insert("lifetime".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("merchant_id".into(), serde_json::to_value(&params.merchant_id).unwrap_or_default());
        body.insert("payment_id".into(), serde_json::to_value(&params.payment_id).unwrap_or_default());
        if let Some(v) = &params.required_telegram_id {
            body.insert("required_telegram_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.required_telegram_username {
            body.insert("required_telegram_username".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.url_callback {
            body.insert("url_callback".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("url_success".into(), serde_json::to_value(&params.url_success).unwrap_or_default());
        self.client.request(
            "post",
            "/invoice",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Invoice
    /// `GET /invoice`
    pub async fn payments_invoice_get(
        &self,
        invoice_id: Option<i64>,
        payment_id: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &invoice_id {
            query.push(("invoice_id", v.to_string()));
        }
        if let Some(v) = &payment_id {
            query.push(("payment_id", v.to_string()));
        }
        self.client.request(
            "get",
            "/invoice",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Invoice List
    /// `GET /invoice/list`
    pub async fn payments_invoice_list(
        &self,
        params: MarketPaymentsInvoiceListParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.status {
            query.push(("status", v.to_string()));
        }
        if let Some(v) = &params.amount {
            query.push(("amount", v.to_string()));
        }
        if let Some(v) = &params.merchant_id {
            query.push(("merchant_id", v.to_string()));
        }
        self.client.request(
            "get",
            "/invoice/list",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Payments ──

    /// Create Auto Payment
    /// `POST /auto-payment`
    pub async fn auto_payments_create(
        &self,
        params: MarketAutoPaymentsCreateParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("amount".into(), serde_json::to_value(&params.amount).unwrap_or_default());
        if let Some(v) = &params.currency {
            body.insert("currency".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("day".into(), serde_json::to_value(&params.day).unwrap_or_default());
        if let Some(v) = &params.description {
            body.insert("description".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.secret_answer {
            body.insert("secret_answer".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("username_receiver".into(), serde_json::to_value(&params.username_receiver).unwrap_or_default());
        self.client.request(
            "post",
            "/auto-payment",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Auto Payment
    /// `DELETE /auto-payment`
    pub async fn auto_payments_delete(
        &self,
        auto_payment_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/auto-payment",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Auto Payments
    /// `GET /auto-payments`
    pub async fn auto_payments_list(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/auto-payments",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get List Of Balances
    /// `GET /balance/exchange`
    pub async fn payments_balance_list(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/balance/exchange",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Exchange Balance
    /// `POST /balance/exchange`
    pub async fn payments_balance_exchange(
        &self,
        amount: i64,
        from_balance: String,
        to_balance: String,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("amount".into(), serde_json::to_value(&amount).unwrap_or_default());
        body.insert("from_balance".into(), serde_json::to_value(&from_balance).unwrap_or_default());
        body.insert("to_balance".into(), serde_json::to_value(&to_balance).unwrap_or_default());
        self.client.request(
            "post",
            "/balance/exchange",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Cancel Transfer
    /// `POST /balance/transfer/cancel`
    pub async fn payments_cancel(
        &self,
        payment_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("payment_id".into(), serde_json::to_value(&payment_id).unwrap_or_default());
        self.client.request(
            "post",
            "/balance/transfer/cancel",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Currency
    /// `GET /currency`
    pub async fn payments_currency(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/currency",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Check Transfer Fee
    /// `GET /balance/transfer/fee`
    pub async fn payments_fee(
        &self,
        amount: Option<f64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &amount {
            query.push(("amount", v.to_string()));
        }
        self.client.request(
            "get",
            "/balance/transfer/fee",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Payments History
    /// `GET /user/payments`
    pub async fn payments_history(
        &self,
        params: MarketPaymentsHistoryParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.r#type {
            query.push(("type", v.to_string()));
        }
        if let Some(v) = &params.pmin {
            query.push(("pmin", v.to_string()));
        }
        if let Some(v) = &params.pmax {
            query.push(("pmax", v.to_string()));
        }
        if let Some(v) = &params.currency {
            query.push(("currency", v.to_string()));
        }
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.operation_id_lt {
            query.push(("operation_id_lt", v.to_string()));
        }
        if let Some(v) = &params.receiver {
            query.push(("receiver", v.to_string()));
        }
        if let Some(v) = &params.sender {
            query.push(("sender", v.to_string()));
        }
        if let Some(v) = &params.is_api {
            query.push(("is_api", v.to_string()));
        }
        if let Some(v) = &params.start_date {
            query.push(("startDate", v.to_string()));
        }
        if let Some(v) = &params.end_date {
            query.push(("endDate", v.to_string()));
        }
        if let Some(v) = &params.wallet {
            query.push(("wallet", v.to_string()));
        }
        if let Some(v) = &params.comment {
            query.push(("comment", v.to_string()));
        }
        if let Some(v) = &params.is_hold {
            query.push(("is_hold", v.to_string()));
        }
        if let Some(v) = &params.show_payment_stats {
            query.push(("show_payment_stats", v.to_string()));
        }
        self.client.request(
            "get",
            "/user/payments",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Create Payout
    /// `POST /balance/payout`
    pub async fn payments_payout(
        &self,
        params: MarketPaymentsPayoutParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("amount".into(), serde_json::to_value(&params.amount).unwrap_or_default());
        body.insert("currency".into(), serde_json::to_value(&params.currency).unwrap_or_default());
        if let Some(v) = &params.extra {
            body.insert("extra".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.include_fee {
            body.insert("include_fee".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("payment_system".into(), serde_json::to_value(&params.payment_system).unwrap_or_default());
        body.insert("wallet".into(), serde_json::to_value(&params.wallet).unwrap_or_default());
        self.client.request(
            "post",
            "/balance/payout",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Payout Services
    /// `GET /balance/payout/services`
    pub async fn payments_payout_services(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/balance/payout/services",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Transfer Money
    /// `POST /balance/transfer`
    pub async fn payments_transfer(
        &self,
        params: MarketPaymentsTransferParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("amount".into(), serde_json::to_value(&params.amount).unwrap_or_default());
        if let Some(v) = &params.comment {
            body.insert("comment".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("currency".into(), serde_json::to_value(&params.currency).unwrap_or_default());
        if let Some(v) = &params.hold_length_option {
            body.insert("hold_length_option".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.hold_length_value {
            body.insert("hold_length_value".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_deal {
            body.insert("telegram_deal".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_username {
            body.insert("telegram_username".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.transfer_hold {
            body.insert("transfer_hold".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user_id {
            body.insert("user_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.username {
            body.insert("username".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/balance/transfer",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }


    // ── Profile ──

    /// Edit Market Settings
    /// `PUT /me`
    pub async fn profile_edit(
        &self,
        params: MarketProfileEditParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_accept_accounts {
            body.insert("allow_accept_accounts".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.clear_telegram_client {
            body.insert("clear_telegram_client".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.option {
            body.insert("option".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_api_hash {
            body.insert("telegram_api_hash".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_api_id {
            body.insert("telegram_api_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_app_version {
            body.insert("telegram_app_version".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_device_model {
            body.insert("telegram_device_model".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_lang_code {
            body.insert("telegram_lang_code".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_lang_pack {
            body.insert("telegram_lang_pack".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_system_lang_code {
            body.insert("telegram_system_lang_code".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.telegram_system_version {
            body.insert("telegram_system_version".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user {
            body.insert("user".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "put",
            "/me",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Profile
    /// `GET /me`
    pub async fn profile_get(
        &self,
        fields_include: Option<Vec<String>>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &fields_include {
            for item in v {
                query.push(("fields_include", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/me",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Proxy ──

    /// Add Proxy
    /// `POST /proxy`
    pub async fn proxy_add(
        &self,
        params: MarketProxyAddParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.proxy_ip {
            body.insert("proxy_ip".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.proxy_pass {
            body.insert("proxy_pass".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.proxy_port {
            body.insert("proxy_port".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.proxy_row {
            body.insert("proxy_row".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.proxy_user {
            body.insert("proxy_user".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/proxy",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Proxy
    /// `DELETE /proxy`
    pub async fn proxy_delete(
        &self,
        delete_all: Option<bool>,
        proxy_id: Option<i64>,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/proxy",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Proxy
    /// `GET /proxy`
    pub async fn proxy_get(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/proxy",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

}
