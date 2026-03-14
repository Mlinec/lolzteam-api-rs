// generated — do not edit



#![allow(unused, clippy::all)]

use crate::client::ApiClient;
use crate::error::Result;
use crate::models::*;
use crate::forum::types::*;

/// Forum API methods.
impl crate::forum::ForumApi {

    // ── Assets ──

    /// Get CSS
    /// `GET /css`
    pub async fn assets_css(
        &self,
        css: Option<Vec<String>>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &css {
            for item in v {
                query.push(("css", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/css",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Authentication ──

    /// Get Access Token
    /// `POST /oauth/token`
    pub async fn o_auth_token(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            "/oauth/token",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Batch requests ──

    /// Batch
    /// `POST /batch`
    pub async fn batch_execute(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            "/batch",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Categories ──

    /// Get Category
    /// `GET /categories/{category_id}`
    pub async fn categories_get(
        &self,
        category_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/categories/{category_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Categories
    /// `GET /categories`
    pub async fn categories_list(
        &self,
        parent_category_id: Option<i64>,
        parent_forum_id: Option<i64>,
        order: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &parent_category_id {
            query.push(("parent_category_id", v.to_string()));
        }
        if let Some(v) = &parent_forum_id {
            query.push(("parent_forum_id", v.to_string()));
        }
        if let Some(v) = &order {
            query.push(("order", v.to_string()));
        }
        self.client.request(
            "get",
            "/categories",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Chatbox ──

    /// Unignore Chat User
    /// `DELETE /chatbox/ignore`
    pub async fn chatbox_delete_ignore(
        &self,
        user_id: serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/chatbox/ignore",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Delete Chat Message
    /// `DELETE /chatbox/messages`
    pub async fn chatbox_delete_message(
        &self,
        message_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/chatbox/messages",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Chat Message
    /// `PUT /chatbox/messages`
    pub async fn chatbox_edit_message(
        &self,
        message: String,
        message_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("message".into(), serde_json::to_value(&message).unwrap_or_default());
        body.insert("message_id".into(), serde_json::to_value(&message_id).unwrap_or_default());
        self.client.request(
            "put",
            "/chatbox/messages",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Ignored Chat Users
    /// `GET /chatbox/ignore`
    pub async fn chatbox_get_ignore(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/chatbox/ignore",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Chat Leaderboard
    /// `GET /chatbox/messages/leaderboard`
    pub async fn chatbox_get_leaderboard(
        &self,
        duration: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &duration {
            query.push(("duration", v.to_string()));
        }
        self.client.request(
            "get",
            "/chatbox/messages/leaderboard",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Chat Messages
    /// `GET /chatbox/messages`
    pub async fn chatbox_get_messages(
        &self,
        room_id: serde_json::Value,
        before_message_id: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        query.push(("room_id", room_id.to_string()));
        if let Some(v) = &before_message_id {
            query.push(("before_message_id", v.to_string()));
        }
        self.client.request(
            "get",
            "/chatbox/messages",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Chats
    /// `GET /chatbox`
    pub async fn chatbox_index(
        &self,
        room_id: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &room_id {
            query.push(("room_id", v.to_string()));
        }
        self.client.request(
            "get",
            "/chatbox",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Chat Online
    /// `GET /chatbox/messages/online`
    pub async fn chatbox_online(
        &self,
        room_id: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        query.push(("room_id", room_id.to_string()));
        self.client.request(
            "get",
            "/chatbox/messages/online",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Ignore Chat User
    /// `POST /chatbox/ignore`
    pub async fn chatbox_post_ignore(
        &self,
        user_id: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("user_id".into(), serde_json::to_value(&user_id).unwrap_or_default());
        self.client.request(
            "post",
            "/chatbox/ignore",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Create Chat Message
    /// `POST /chatbox/messages`
    pub async fn chatbox_post_message(
        &self,
        message: String,
        room_id: serde_json::Value,
        reply_message_id: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("message".into(), serde_json::to_value(&message).unwrap_or_default());
        if let Some(v) = &reply_message_id {
            body.insert("reply_message_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("room_id".into(), serde_json::to_value(&room_id).unwrap_or_default());
        self.client.request(
            "post",
            "/chatbox/messages",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Report Chat Message
    /// `POST /chatbox/messages/report`
    pub async fn chatbox_report(
        &self,
        message_id: i64,
        reason: String,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("message_id".into(), serde_json::to_value(&message_id).unwrap_or_default());
        body.insert("reason".into(), serde_json::to_value(&reason).unwrap_or_default());
        self.client.request(
            "post",
            "/chatbox/messages/report",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Chat Message Report Reasons
    /// `GET /chatbox/messages/report`
    pub async fn chatbox_report_reasons(
        &self,
        message_id: i64,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        query.push(("message_id", message_id.to_string()));
        self.client.request(
            "get",
            "/chatbox/messages/report",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Content Tagging ──

    /// Get Filtered Content
    /// `GET /tags/find`
    pub async fn tags_find(
        &self,
        tag: String,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        query.push(("tag", tag.to_string()));
        self.client.request(
            "get",
            "/tags/find",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Tagged Content
    /// `GET /tags/{tag_id}`
    pub async fn tags_get(
        &self,
        tag_id: i64,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/tags/{tag_id}"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Tags
    /// `GET /tags/list`
    pub async fn tags_list(
        &self,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            "/tags/list",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Popular Tags
    /// `GET /tags`
    pub async fn tags_popular(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/tags",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Conversations ──

    /// Disable Conversation Alerts
    /// `DELETE /conversations/{conversation_id}/alerts`
    pub async fn conversations_alerts_disable(
        &self,
        conversation_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/conversations/{conversation_id}/alerts"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Enable Conversation Alerts
    /// `POST /conversations/{conversation_id}/alerts`
    pub async fn conversations_alerts_enable(
        &self,
        conversation_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/conversations/{conversation_id}/alerts"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Create Conversation
    /// `POST /conversations`
    pub async fn conversations_create(
        &self,
        params: ForumConversationsCreateParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_delete_own_messages {
            body.insert("allow_delete_own_messages".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.allow_edit_messages {
            body.insert("allow_edit_messages".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.allow_sticky_messages {
            body.insert("allow_sticky_messages".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.is_group {
            body.insert("is_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.message_body {
            body.insert("message_body".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.open_invite {
            body.insert("open_invite".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.recipient_id {
            body.insert("recipient_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.recipients {
            body.insert("recipients".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title {
            body.insert("title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/conversations",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Leave Conversation
    /// `DELETE /conversations`
    pub async fn conversations_delete(
        &self,
        conversation_id: i64,
        delete_type: String,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/conversations",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Conversation
    /// `GET /conversations/{conversation_id}`
    pub async fn conversations_get(
        &self,
        conversation_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/conversations/{conversation_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Invite Users to Conversation
    /// `POST /conversations/{conversation_id}/invite`
    pub async fn conversations_invite(
        &self,
        conversation_id: i64,
        recipients: Vec<String>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("recipients".into(), serde_json::to_value(&recipients).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/conversations/{conversation_id}/invite"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Kick User from Conversation
    /// `POST /conversations/{conversation_id}/kick`
    pub async fn conversations_kick(
        &self,
        conversation_id: i64,
        user_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("user_id".into(), serde_json::to_value(&user_id).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/conversations/{conversation_id}/kick"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Conversations
    /// `GET /conversations`
    pub async fn conversations_list(
        &self,
        folder: Option<String>,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &folder {
            query.push(("folder", v.to_string()));
        }
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            "/conversations",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Create Conversation Message
    /// `POST /conversations/{conversation_id}/messages`
    pub async fn conversations_messages_create(
        &self,
        conversation_id: i64,
        message_body: String,
        reply_message_id: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("message_body".into(), serde_json::to_value(&message_body).unwrap_or_default());
        if let Some(v) = &reply_message_id {
            body.insert("reply_message_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/conversations/{conversation_id}/messages"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Conversation Message
    /// `DELETE /conversations/{conversation_id}/messages/{message_id}`
    pub async fn conversations_messages_delete(
        &self,
        conversation_id: i64,
        message_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/conversations/{conversation_id}/messages/{message_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Conversation Message
    /// `PUT /conversations/{conversation_id}/messages/{message_id}`
    pub async fn conversations_messages_edit(
        &self,
        conversation_id: i64,
        message_id: i64,
        message_body: String,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("message_body".into(), serde_json::to_value(&message_body).unwrap_or_default());
        self.client.request(
            "put",
            &format!("/conversations/{conversation_id}/messages/{message_id}"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Conversation Message
    /// `GET /conversations/messages/{message_id}`
    pub async fn conversations_messages_get(
        &self,
        message_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/conversations/messages/{message_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Conversation Messages
    /// `GET /conversations/{conversation_id}/messages`
    pub async fn conversations_messages_list(
        &self,
        conversation_id: i64,
        params: ForumConversationsMessagesListParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.limit {
            query.push(("limit", v.to_string()));
        }
        if let Some(v) = &params.order {
            query.push(("order", v.to_string()));
        }
        if let Some(v) = &params.before {
            query.push(("before", v.to_string()));
        }
        if let Some(v) = &params.after {
            query.push(("after", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/conversations/{conversation_id}/messages"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Stick Conversation Message
    /// `POST /conversations/{conversation_id}/messages/{message_id}/stick`
    pub async fn conversations_messages_stick(
        &self,
        conversation_id: i64,
        message_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/conversations/{conversation_id}/messages/{message_id}/stick"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Unstick Conversation Message
    /// `DELETE /conversations/{conversation_id}/messages/{message_id}/stick`
    pub async fn conversations_messages_unstick(
        &self,
        conversation_id: i64,
        message_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/conversations/{conversation_id}/messages/{message_id}/stick"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Read a Conversation
    /// `POST /conversations/{conversation_id}/read`
    pub async fn conversations_read(
        &self,
        conversation_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/conversations/{conversation_id}/read"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Read All Conversations
    /// `POST /conversations/read-all`
    pub async fn conversations_read_all(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            "/conversations/read-all",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Send Content To Saved Messages
    /// `POST /conversations/save`
    pub async fn conversations_save(
        &self,
        link: String,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("link".into(), serde_json::to_value(&link).unwrap_or_default());
        self.client.request(
            "post",
            "/conversations/save",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Search Conversations Messages
    /// `POST /conversations/search`
    pub async fn conversations_search(
        &self,
        conversation_id: Option<i64>,
        q: Option<String>,
        search_recipients: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &conversation_id {
            body.insert("conversation_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &q {
            body.insert("q".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &search_recipients {
            body.insert("search_recipients".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/conversations/search",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Star Conversation
    /// `POST /conversations/{conversation_id}/star`
    pub async fn conversations_star(
        &self,
        conversation_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/conversations/{conversation_id}/star"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Start Conversation
    /// `POST /conversations/start`
    pub async fn conversations_start(
        &self,
        user_id: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("user_id".into(), serde_json::to_value(&user_id).unwrap_or_default());
        self.client.request(
            "post",
            "/conversations/start",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Unstar Conversation
    /// `DELETE /conversations/{conversation_id}/star`
    pub async fn conversations_unstar(
        &self,
        conversation_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/conversations/{conversation_id}/star"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Conversation
    /// `PUT /conversations`
    pub async fn conversations_update(
        &self,
        params: ForumConversationsUpdateParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_delete_own_messages {
            body.insert("allow_delete_own_messages".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.allow_edit_messages {
            body.insert("allow_edit_messages".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.allow_sticky_messages {
            body.insert("allow_sticky_messages".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("conversation_id".into(), serde_json::to_value(&params.conversation_id).unwrap_or_default());
        if let Some(v) = &params.history_open {
            body.insert("history_open".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.open_invite {
            body.insert("open_invite".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title {
            body.insert("title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "put",
            "/conversations",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }


    // ── Forms ──

    /// Create Form
    /// `POST /forms/save`
    pub async fn forms_create(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            "/forms/save",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Forms List
    /// `GET /forms`
    pub async fn forms_list(
        &self,
        page: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        self.client.request(
            "get",
            "/forms",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Forums ──

    /// Edit Feed Options
    /// `PUT /forums/feed/options`
    pub async fn forums_edit_feed_options(
        &self,
        keywords: Option<Vec<String>>,
        node_ids: Option<Vec<i64>>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &keywords {
            body.insert("keywords".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &node_ids {
            body.insert("node_ids".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "put",
            "/forums/feed/options",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Follow Forum
    /// `POST /forums/{forum_id}/followers`
    pub async fn forums_follow(
        &self,
        forum_id: i64,
        params: ForumForumsFollowParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.alert {
            body.insert("alert".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.email {
            body.insert("email".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.minimal_contest_amount {
            body.insert("minimal_contest_amount".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.post {
            body.insert("post".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.prefix_ids {
            body.insert("prefix_ids".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/forums/{forum_id}/followers"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Followed Forums
    /// `GET /forums/followed`
    pub async fn forums_followed(
        &self,
        total: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &total {
            query.push(("total", v.to_string()));
        }
        self.client.request(
            "get",
            "/forums/followed",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Followers
    /// `GET /forums/{forum_id}/followers`
    pub async fn forums_followers(
        &self,
        forum_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/forums/{forum_id}/followers"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Forum
    /// `GET /forums/{forum_id}`
    pub async fn forums_get(
        &self,
        forum_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/forums/{forum_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Feed Options
    /// `GET /forums/feed/options`
    pub async fn forums_get_feed_options(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/forums/feed/options",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Forums Tree
    /// `GET /forums/grouped`
    pub async fn forums_grouped(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/forums/grouped",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Forums
    /// `GET /forums`
    pub async fn forums_list(
        &self,
        parent_category_id: Option<i64>,
        parent_forum_id: Option<i64>,
        order: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &parent_category_id {
            query.push(("parent_category_id", v.to_string()));
        }
        if let Some(v) = &parent_forum_id {
            query.push(("parent_forum_id", v.to_string()));
        }
        if let Some(v) = &order {
            query.push(("order", v.to_string()));
        }
        self.client.request(
            "get",
            "/forums",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Unfollow Forum
    /// `DELETE /forums/{forum_id}/followers`
    pub async fn forums_unfollow(
        &self,
        forum_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/forums/{forum_id}/followers"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Link Forums ──

    /// Get Link Forum
    /// `GET /link-forums/{link_id}`
    pub async fn links_get(
        &self,
        link_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/link-forums/{link_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Links Forum
    /// `GET /link-forums`
    pub async fn links_list(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/link-forums",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Navigation ──

    /// Get Navigation
    /// `GET /navigation`
    pub async fn navigation_list(
        &self,
        parent: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &parent {
            query.push(("parent", v.to_string()));
        }
        self.client.request(
            "get",
            "/navigation",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Notifications ──

    /// Get Notification
    /// `GET /notifications/{notification_id}/content`
    pub async fn notifications_get(
        &self,
        notification_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/notifications/{notification_id}/content"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Notifications
    /// `GET /notifications`
    pub async fn notifications_list(
        &self,
        r#type: Option<String>,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &r#type {
            query.push(("type", v.to_string()));
        }
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            "/notifications",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Mark Notification Read
    /// `POST /notifications/read`
    pub async fn notifications_read(
        &self,
        notification_id: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &notification_id {
            body.insert("notification_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/notifications/read",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }


    // ── Pages ──

    /// Get Page
    /// `GET /pages/{page_id}`
    pub async fn pages_get(
        &self,
        page_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/pages/{page_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Pages
    /// `GET /pages`
    pub async fn pages_list(
        &self,
        parent_page_id: Option<i64>,
        order: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &parent_page_id {
            query.push(("parent_page_id", v.to_string()));
        }
        if let Some(v) = &order {
            query.push(("order", v.to_string()));
        }
        self.client.request(
            "get",
            "/pages",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Post comments ──

    /// Create Post Comment
    /// `POST /posts/comments`
    pub async fn posts_comments_create(
        &self,
        comment_body: String,
        post_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("comment_body".into(), serde_json::to_value(&comment_body).unwrap_or_default());
        body.insert("post_id".into(), serde_json::to_value(&post_id).unwrap_or_default());
        self.client.request(
            "post",
            "/posts/comments",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Post Comment
    /// `DELETE /posts/comments`
    pub async fn posts_comments_delete(
        &self,
        post_comment_id: i64,
        reason: Option<String>,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/posts/comments",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Post Comment
    /// `PUT /posts/comments`
    pub async fn posts_comments_edit(
        &self,
        comment_body: String,
        post_comment_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("comment_body".into(), serde_json::to_value(&comment_body).unwrap_or_default());
        body.insert("post_comment_id".into(), serde_json::to_value(&post_comment_id).unwrap_or_default());
        self.client.request(
            "put",
            "/posts/comments",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Post Comments
    /// `GET /posts/comments`
    pub async fn posts_comments_get(
        &self,
        post_id: i64,
        before: Option<i64>,
        before_comment: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        query.push(("post_id", post_id.to_string()));
        if let Some(v) = &before {
            query.push(("before", v.to_string()));
        }
        if let Some(v) = &before_comment {
            query.push(("before_comment", v.to_string()));
        }
        self.client.request(
            "get",
            "/posts/comments",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }


    // ── Posts ──

    /// Report Post Comment
    /// `POST /posts/comments/report`
    pub async fn posts_comments_report(
        &self,
        message: String,
        post_comment_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("message".into(), serde_json::to_value(&message).unwrap_or_default());
        body.insert("post_comment_id".into(), serde_json::to_value(&post_comment_id).unwrap_or_default());
        self.client.request(
            "post",
            "/posts/comments/report",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Create Post
    /// `POST /posts`
    pub async fn posts_create(
        &self,
        post_body: String,
        quote_post_id: Option<i64>,
        thread_id: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("post_body".into(), serde_json::to_value(&post_body).unwrap_or_default());
        if let Some(v) = &quote_post_id {
            body.insert("quote_post_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &thread_id {
            body.insert("thread_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/posts",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Post
    /// `DELETE /posts/{post_id}`
    pub async fn posts_delete(
        &self,
        post_id: i64,
        reason: Option<String>,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/posts/{post_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Post
    /// `PUT /posts/{post_id}`
    pub async fn posts_edit(
        &self,
        post_id: i64,
        post_body: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &post_body {
            body.insert("post_body".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "put",
            &format!("/posts/{post_id}"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Post
    /// `GET /posts/{post_id}`
    pub async fn posts_get(
        &self,
        post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/posts/{post_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Like Post
    /// `POST /posts/{post_id}/likes`
    pub async fn posts_like(
        &self,
        post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/posts/{post_id}/likes"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Post Likes
    /// `GET /posts/{post_id}/likes`
    pub async fn posts_likes(
        &self,
        post_id: i64,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/posts/{post_id}/likes"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Posts
    /// `GET /posts`
    pub async fn posts_list(
        &self,
        params: ForumPostsListParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.thread_id {
            query.push(("thread_id", v.to_string()));
        }
        if let Some(v) = &params.page_of_post_id {
            query.push(("page_of_post_id", v.to_string()));
        }
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.limit {
            query.push(("limit", v.to_string()));
        }
        if let Some(v) = &params.order {
            query.push(("order", v.to_string()));
        }
        self.client.request(
            "get",
            "/posts",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Report Post
    /// `POST /posts/{post_id}/report`
    pub async fn posts_report(
        &self,
        post_id: i64,
        message: String,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("message".into(), serde_json::to_value(&message).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/posts/{post_id}/report"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Post Report Reasons
    /// `GET /posts/{post_id}/report`
    pub async fn posts_report_reasons(
        &self,
        post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/posts/{post_id}/report"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Unlike Post
    /// `DELETE /posts/{post_id}/likes`
    pub async fn posts_unlike(
        &self,
        post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/posts/{post_id}/likes"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Profile Post Comments ──

    /// Create Profile Post Comment
    /// `POST /profile-posts/comments`
    pub async fn profile_posts_comments_create(
        &self,
        comment_body: String,
        profile_post_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("comment_body".into(), serde_json::to_value(&comment_body).unwrap_or_default());
        body.insert("profile_post_id".into(), serde_json::to_value(&profile_post_id).unwrap_or_default());
        self.client.request(
            "post",
            "/profile-posts/comments",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Profile Post Comment
    /// `DELETE /profile-posts/comments`
    pub async fn profile_posts_comments_delete(
        &self,
        comment_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/profile-posts/comments",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Profile Post Comment
    /// `PUT /profile-posts/comments`
    pub async fn profile_posts_comments_edit(
        &self,
        comment_body: String,
        comment_id: i64,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("comment_body".into(), serde_json::to_value(&comment_body).unwrap_or_default());
        body.insert("comment_id".into(), serde_json::to_value(&comment_id).unwrap_or_default());
        self.client.request(
            "put",
            "/profile-posts/comments",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Profile Post Comment
    /// `GET /profile-posts/{profile_post_id}/comments/{comment_id}`
    pub async fn profile_posts_comments_get(
        &self,
        profile_post_id: i64,
        comment_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/profile-posts/{profile_post_id}/comments/{comment_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Profile Post Comments
    /// `GET /profile-posts/comments`
    pub async fn profile_posts_comments_list(
        &self,
        profile_post_id: i64,
        before: Option<i64>,
        limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        query.push(("profile_post_id", profile_post_id.to_string()));
        if let Some(v) = &before {
            query.push(("before", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            "/profile-posts/comments",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Report a Profile Post Comment
    /// `POST /profile-posts/comments/{comment_id}/report`
    pub async fn profile_posts_comments_report(
        &self,
        comment_id: i64,
        message: String,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("message".into(), serde_json::to_value(&message).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/profile-posts/comments/{comment_id}/report"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }


    // ── Profile Posts ──

    /// Create Profile Post
    /// `POST /profile-posts`
    pub async fn profile_posts_create(
        &self,
        post_body: String,
        user_id: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("post_body".into(), serde_json::to_value(&post_body).unwrap_or_default());
        body.insert("user_id".into(), serde_json::to_value(&user_id).unwrap_or_default());
        self.client.request(
            "post",
            "/profile-posts",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Profile Post
    /// `DELETE /profile-posts/{profile_post_id}`
    pub async fn profile_posts_delete(
        &self,
        profile_post_id: i64,
        reason: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &reason {
            query.push(("reason", v.to_string()));
        }
        self.client.request(
            "delete",
            &format!("/profile-posts/{profile_post_id}"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Profile Post
    /// `PUT /profile-posts/{profile_post_id}`
    pub async fn profile_posts_edit(
        &self,
        profile_post_id: i64,
        disable_comments: Option<bool>,
        post_body: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &disable_comments {
            body.insert("disable_comments".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &post_body {
            body.insert("post_body".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "put",
            &format!("/profile-posts/{profile_post_id}"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Profile Post
    /// `GET /profile-posts/{profile_post_id}`
    pub async fn profile_posts_get(
        &self,
        profile_post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/profile-posts/{profile_post_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Like Profile Post
    /// `POST /profile-posts/{profile_post_id}/likes`
    pub async fn profile_posts_like(
        &self,
        profile_post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/profile-posts/{profile_post_id}/likes"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Profile Post Likes
    /// `GET /profile-posts/{profile_post_id}/likes`
    pub async fn profile_posts_likes(
        &self,
        profile_post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/profile-posts/{profile_post_id}/likes"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Profile Posts
    /// `GET /users/{user_id}/profile-posts`
    pub async fn profile_posts_list(
        &self,
        user_id: i64,
        params: ForumProfilePostsListParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.posts_user_id {
            query.push(("posts_user_id", v.to_string()));
        }
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.limit {
            query.push(("limit", v.to_string()));
        }
        if let Some(v) = &params.fields_include {
            for item in v {
                query.push(("fields_include", item.to_string()));
            }
        }
        self.client.request(
            "get",
            &format!("/users/{user_id}/profile-posts"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Report a Profile Post
    /// `POST /profile-posts/{profile_post_id}/report`
    pub async fn profile_posts_report(
        &self,
        profile_post_id: i64,
        message: String,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("message".into(), serde_json::to_value(&message).unwrap_or_default());
        self.client.request(
            "post",
            &format!("/profile-posts/{profile_post_id}/report"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Profile Post Report Reasons
    /// `GET /profile-posts/{profile_post_id}/report`
    pub async fn profile_posts_report_reasons(
        &self,
        profile_post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/profile-posts/{profile_post_id}/report"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Stick Profile Post
    /// `POST /profile-posts/{profile_post_id}/stick`
    pub async fn profile_posts_stick(
        &self,
        profile_post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/profile-posts/{profile_post_id}/stick"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Unlike Profile Post
    /// `DELETE /profile-posts/{profile_post_id}/likes`
    pub async fn profile_posts_unlike(
        &self,
        profile_post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/profile-posts/{profile_post_id}/likes"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Unstick Profile Post
    /// `DELETE /profile-posts/{profile_post_id}/stick`
    pub async fn profile_posts_unstick(
        &self,
        profile_post_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/profile-posts/{profile_post_id}/stick"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Searching ──

    /// Search
    /// `POST /search`
    pub async fn search_all(
        &self,
        params: ForumSearchAllParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.forum_id {
            body.insert("forum_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.limit {
            body.insert("limit".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.page {
            body.insert("page".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.q {
            body.insert("q".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.tag {
            body.insert("tag".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user_id {
            body.insert("user_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/search",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Search Post
    /// `POST /search/posts`
    pub async fn search_posts(
        &self,
        params: ForumSearchPostsParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.data_limit {
            body.insert("data_limit".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.forum_id {
            body.insert("forum_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.limit {
            body.insert("limit".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.page {
            body.insert("page".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.q {
            body.insert("q".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.tag {
            body.insert("tag".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user_id {
            body.insert("user_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/search/posts",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Search Profile Posts
    /// `POST /search/profile-posts`
    pub async fn search_profile_posts(
        &self,
        params: ForumSearchProfilePostsParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.limit {
            body.insert("limit".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.page {
            body.insert("page".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.q {
            body.insert("q".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user_id {
            body.insert("user_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/search/profile-posts",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Search Results
    /// `GET /search/{search_id}/results`
    pub async fn search_results(
        &self,
        search_id: String,
        limit: Option<i64>,
        page: Option<i64>,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/search/{search_id}/results"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Search Tagged
    /// `POST /search/tagged`
    pub async fn search_tagged(
        &self,
        params: ForumSearchTaggedParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.limit {
            body.insert("limit".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.page {
            body.insert("page".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.tag {
            body.insert("tag".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.tags {
            body.insert("tags".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/search/tagged",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Search Thread
    /// `POST /search/threads`
    pub async fn search_threads(
        &self,
        params: ForumSearchThreadsParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.data_limit {
            body.insert("data_limit".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.forum_id {
            body.insert("forum_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.limit {
            body.insert("limit".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.page {
            body.insert("page".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.q {
            body.insert("q".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.tag {
            body.insert("tag".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user_id {
            body.insert("user_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/search/threads",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Search Users
    /// `POST /search/users`
    pub async fn search_users(
        &self,
        q: Option<String>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &q {
            body.insert("q".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/search/users",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }


    // ── Threads ──

    /// Bump Thread
    /// `POST /threads/{thread_id}/bump`
    pub async fn threads_bump(
        &self,
        thread_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/threads/{thread_id}/bump"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Create Claim
    /// `POST /claims`
    pub async fn threads_claim(
        &self,
        params: ForumThreadsClaimParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_ask_hidden_content {
            body.insert("allow_ask_hidden_content".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("as_amount".into(), serde_json::to_value(&params.as_amount).unwrap_or_default());
        if let Some(v) = &params.as_data {
            body.insert("as_data".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.as_funds_receipt {
            body.insert("as_funds_receipt".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("as_is_market_deal".into(), serde_json::to_value(&params.as_is_market_deal).unwrap_or_default());
        if let Some(v) = &params.as_market_item_id {
            body.insert("as_market_item_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("as_responder".into(), serde_json::to_value(&params.as_responder).unwrap_or_default());
        if let Some(v) = &params.as_tg_login_screenshot {
            body.insert("as_tg_login_screenshot".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.comment_ignore_group {
            body.insert("comment_ignore_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.currency {
            body.insert("currency".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.dont_alert_followers {
            body.insert("dont_alert_followers".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.hide_contacts {
            body.insert("hide_contacts".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.pay_claim {
            body.insert("pay_claim".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("post_body".into(), serde_json::to_value(&params.post_body).unwrap_or_default());
        if let Some(v) = &params.reply_group {
            body.insert("reply_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.schedule_date {
            body.insert("schedule_date".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.schedule_time {
            body.insert("schedule_time".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.tags {
            body.insert("tags".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("transfer_type".into(), serde_json::to_value(&params.transfer_type).unwrap_or_default());
        if let Some(v) = &params.watch_thread {
            body.insert("watch_thread".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.watch_thread_email {
            body.insert("watch_thread_email".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.watch_thread_state {
            body.insert("watch_thread_state".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/claims",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Create Thread
    /// `POST /threads`
    pub async fn threads_create(
        &self,
        params: ForumThreadsCreateParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_ask_hidden_content {
            body.insert("allow_ask_hidden_content".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.comment_ignore_group {
            body.insert("comment_ignore_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.dont_alert_followers {
            body.insert("dont_alert_followers".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("forum_id".into(), serde_json::to_value(&params.forum_id).unwrap_or_default());
        if let Some(v) = &params.hide_contacts {
            body.insert("hide_contacts".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("post_body".into(), serde_json::to_value(&params.post_body).unwrap_or_default());
        if let Some(v) = &params.prefix_id {
            body.insert("prefix_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.reply_group {
            body.insert("reply_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.schedule_date {
            body.insert("schedule_date".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.schedule_time {
            body.insert("schedule_time".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.tags {
            body.insert("tags".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title {
            body.insert("title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title_en {
            body.insert("title_en".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.watch_thread {
            body.insert("watch_thread".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.watch_thread_email {
            body.insert("watch_thread_email".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.watch_thread_state {
            body.insert("watch_thread_state".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/threads",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Create Contest
    /// `POST /contests`
    pub async fn threads_create_contest(
        &self,
        params: ForumThreadsCreateContestParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_ask_hidden_content {
            body.insert("allow_ask_hidden_content".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.comment_ignore_group {
            body.insert("comment_ignore_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("contest_type".into(), serde_json::to_value(&params.contest_type).unwrap_or_default());
        if let Some(v) = &params.count_winners {
            body.insert("count_winners".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.dont_alert_followers {
            body.insert("dont_alert_followers".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.hide_contacts {
            body.insert("hide_contacts".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.is_money_places {
            body.insert("is_money_places".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.length_option {
            body.insert("length_option".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.length_value {
            body.insert("length_value".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("post_body".into(), serde_json::to_value(&params.post_body).unwrap_or_default());
        if let Some(v) = &params.prize_data_money {
            body.insert("prize_data_money".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.prize_data_places {
            body.insert("prize_data_places".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.prize_data_upgrade {
            body.insert("prize_data_upgrade".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("prize_type".into(), serde_json::to_value(&params.prize_type).unwrap_or_default());
        if let Some(v) = &params.reply_group {
            body.insert("reply_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("require_like_count".into(), serde_json::to_value(&params.require_like_count).unwrap_or_default());
        body.insert("require_total_like_count".into(), serde_json::to_value(&params.require_total_like_count).unwrap_or_default());
        if let Some(v) = &params.schedule_date {
            body.insert("schedule_date".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.schedule_time {
            body.insert("schedule_time".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.secret_answer {
            body.insert("secret_answer".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.tags {
            body.insert("tags".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title {
            body.insert("title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title_en {
            body.insert("title_en".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.watch_thread {
            body.insert("watch_thread".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.watch_thread_email {
            body.insert("watch_thread_email".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.watch_thread_state {
            body.insert("watch_thread_state".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            "/contests",
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Thread
    /// `DELETE /threads/{thread_id}`
    pub async fn threads_delete(
        &self,
        thread_id: i64,
        reason: Option<String>,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/threads/{thread_id}"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Edit thread
    /// `PUT /threads/{thread_id}`
    pub async fn threads_edit(
        &self,
        thread_id: i64,
        params: ForumThreadsEditParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.allow_ask_hidden_content {
            body.insert("allow_ask_hidden_content".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.comment_ignore_group {
            body.insert("comment_ignore_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.discussion_open {
            body.insert("discussion_open".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.hide_contacts {
            body.insert("hide_contacts".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.prefix_id {
            body.insert("prefix_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.reply_group {
            body.insert("reply_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.tags {
            body.insert("tags".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title {
            body.insert("title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title_en {
            body.insert("title_en".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "put",
            &format!("/threads/{thread_id}"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Finish Contest
    /// `POST /contests/{thread_id}/finish`
    pub async fn threads_finish(
        &self,
        thread_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/contests/{thread_id}/finish"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Follow Thread
    /// `POST /threads/{thread_id}/followers`
    pub async fn threads_follow(
        &self,
        thread_id: i64,
        email: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &email {
            body.insert("email".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/threads/{thread_id}/followers"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Followed Threads
    /// `GET /threads/followed`
    pub async fn threads_followed(
        &self,
        total: Option<bool>,
        fields_include: Option<Vec<String>>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &total {
            query.push(("total", v.to_string()));
        }
        if let Some(v) = &fields_include {
            for item in v {
                query.push(("fields_include", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/threads/followed",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Thread Followers
    /// `GET /threads/{thread_id}/followers`
    pub async fn threads_followers(
        &self,
        thread_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/threads/{thread_id}/followers"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Thread
    /// `GET /threads/{thread_id}`
    pub async fn threads_get(
        &self,
        thread_id: i64,
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
            &format!("/threads/{thread_id}"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Hide Thread
    /// `POST /threads/{thread_id}/hide`
    pub async fn threads_hide(
        &self,
        thread_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/threads/{thread_id}/hide"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Threads
    /// `GET /threads`
    pub async fn threads_list(
        &self,
        params: ForumThreadsListParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.forum_id {
            query.push(("forum_id", v.to_string()));
        }
        if let Some(v) = &params.tab {
            query.push(("tab", v.to_string()));
        }
        if let Some(v) = &params.state {
            query.push(("state", v.to_string()));
        }
        if let Some(v) = &params.period {
            query.push(("period", v.to_string()));
        }
        if let Some(v) = &params.title {
            query.push(("title", v.to_string()));
        }
        if let Some(v) = &params.title_only {
            query.push(("title_only", v.to_string()));
        }
        if let Some(v) = &params.creator_user_id {
            query.push(("creator_user_id", v.to_string()));
        }
        if let Some(v) = &params.sticky {
            query.push(("sticky", v.to_string()));
        }
        if let Some(v) = &params.prefix_ids {
            for item in v {
                query.push(("prefix_ids[]", item.to_string()));
            }
        }
        if let Some(v) = &params.prefix_ids_not {
            for item in v {
                query.push(("prefix_ids_not[]", item.to_string()));
            }
        }
        if let Some(v) = &params.thread_tag_id {
            query.push(("thread_tag_id", v.to_string()));
        }
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.limit {
            query.push(("limit", v.to_string()));
        }
        if let Some(v) = &params.order {
            query.push(("order", v.to_string()));
        }
        if let Some(v) = &params.direction {
            query.push(("direction", v.to_string()));
        }
        if let Some(v) = &params.thread_create_date {
            query.push(("thread_create_date", v.to_string()));
        }
        if let Some(v) = &params.thread_update_date {
            query.push(("thread_update_date", v.to_string()));
        }
        if let Some(v) = &params.fields_include {
            for item in v {
                query.push(("fields_include", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/threads",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Move Thread
    /// `POST /threads/{thread_id}/move`
    pub async fn threads_move(
        &self,
        thread_id: i64,
        params: ForumThreadsMoveParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.apply_thread_prefix {
            body.insert("apply_thread_prefix".into(), serde_json::to_value(v).unwrap_or_default());
        }
        body.insert("node_id".into(), serde_json::to_value(&params.node_id).unwrap_or_default());
        if let Some(v) = &params.prefix_id {
            body.insert("prefix_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.send_alert {
            body.insert("send_alert".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title {
            body.insert("title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.title_en {
            body.insert("title_en".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/threads/{thread_id}/move"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Navigation Elements
    /// `GET /threads/{thread_id}/navigation`
    pub async fn threads_navigation(
        &self,
        thread_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/threads/{thread_id}/navigation"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Poll
    /// `GET /threads/{thread_id}/poll`
    pub async fn threads_poll_get(
        &self,
        thread_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/threads/{thread_id}/poll"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Vote Poll
    /// `POST /threads/{thread_id}/poll/votes`
    pub async fn threads_poll_vote(
        &self,
        thread_id: i64,
        response_id: Option<i64>,
        response_ids: Option<Vec<i64>>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &response_id {
            body.insert("response_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &response_ids {
            body.insert("response_ids".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/threads/{thread_id}/poll/votes"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get Recent Threads
    /// `GET /threads/recent`
    pub async fn threads_recent(
        &self,
        params: ForumThreadsRecentParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.days {
            query.push(("days", v.to_string()));
        }
        if let Some(v) = &params.limit {
            query.push(("limit", v.to_string()));
        }
        if let Some(v) = &params.forum_id {
            query.push(("forum_id", v.to_string()));
        }
        if let Some(v) = &params.data_limit {
            query.push(("data_limit", v.to_string()));
        }
        self.client.request(
            "get",
            "/threads/recent",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Bookmark Thread
    /// `POST /threads/{thread_id}/star`
    pub async fn threads_star(
        &self,
        thread_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/threads/{thread_id}/star"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Unfollow Thread
    /// `DELETE /threads/{thread_id}/followers`
    pub async fn threads_unfollow(
        &self,
        thread_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/threads/{thread_id}/followers"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Unread Threads
    /// `GET /threads/new`
    pub async fn threads_unread(
        &self,
        limit: Option<i64>,
        forum_id: Option<i64>,
        data_limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        if let Some(v) = &forum_id {
            query.push(("forum_id", v.to_string()));
        }
        if let Some(v) = &data_limit {
            query.push(("data_limit", v.to_string()));
        }
        self.client.request(
            "get",
            "/threads/new",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Unbookmark Thread
    /// `DELETE /threads/{thread_id}/star`
    pub async fn threads_unstar(
        &self,
        thread_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/threads/{thread_id}/star"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }


    // ── Users ──

    /// Crop Avatar
    /// `POST /users/{user_id}/avatar/crop`
    pub async fn users_avatar_crop(
        &self,
        user_id: i64,
        crop: Option<i64>,
        x: Option<i64>,
        y: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &crop {
            body.insert("crop".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &x {
            body.insert("x".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &y {
            body.insert("y".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/users/{user_id}/avatar/crop"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Avatar
    /// `DELETE /users/{user_id}/avatar`
    pub async fn users_avatar_delete(
        &self,
        user_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/users/{user_id}/avatar"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Upload Avatar
    /// `POST /users/{user_id}/avatar`
    pub async fn users_avatar_upload(
        &self,
        user_id: i64,
        params: ForumUsersAvatarUploadParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("avatar".into(), serde_json::to_value(&params.avatar).unwrap_or_default());
        if let Some(v) = &params.crop {
            body.insert("crop".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.x {
            body.insert("x".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.y {
            body.insert("y".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/users/{user_id}/avatar"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Crop Background
    /// `POST /users/{user_id}/background/crop`
    pub async fn users_background_crop(
        &self,
        user_id: i64,
        crop: Option<i64>,
        x: Option<i64>,
        y: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &crop {
            body.insert("crop".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &x {
            body.insert("x".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &y {
            body.insert("y".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/users/{user_id}/background/crop"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Delete Background
    /// `DELETE /users/{user_id}/background`
    pub async fn users_background_delete(
        &self,
        user_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/users/{user_id}/background"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Upload Background
    /// `POST /users/{user_id}/background`
    pub async fn users_background_upload(
        &self,
        user_id: i64,
        params: ForumUsersBackgroundUploadParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        body.insert("background".into(), serde_json::to_value(&params.background).unwrap_or_default());
        if let Some(v) = &params.crop {
            body.insert("crop".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.x {
            body.insert("x".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.y {
            body.insert("y".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "post",
            &format!("/users/{user_id}/background"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get User Claims
    /// `GET /users/{user_id}/claims`
    pub async fn users_claims(
        &self,
        user_id: i64,
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
            &format!("/users/{user_id}/claims"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Contents
    /// `GET /users/{user_id}/timeline`
    pub async fn users_contents(
        &self,
        user_id: i64,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/users/{user_id}/timeline"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Edit User
    /// `PUT /users/{user_id}`
    pub async fn users_edit(
        &self,
        user_id: i64,
        params: ForumUsersEditParams,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::Map::new();
        if let Some(v) = &params.activity_visible {
            body.insert("activity_visible".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.alert {
            body.insert("alert".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.allow_invite_group {
            body.insert("allow_invite_group".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.allow_post_profile {
            body.insert("allow_post_profile".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.allow_receive_news_feed {
            body.insert("allow_receive_news_feed".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.allow_send_personal_conversation {
            body.insert("allow_send_personal_conversation".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.allow_view_profile {
            body.insert("allow_view_profile".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.conv_welcome_message {
            body.insert("conv_welcome_message".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.display_banner_id {
            body.insert("display_banner_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.display_group_id {
            body.insert("display_group_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.display_icon_group_id {
            body.insert("display_icon_group_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.fields {
            body.insert("fields".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.gender {
            body.insert("gender".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.hide_username_change_logs {
            body.insert("hide_username_change_logs".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.language_id {
            body.insert("language_id".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.receive_admin_email {
            body.insert("receive_admin_email".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.secret_answer {
            body.insert("secret_answer".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.secret_answer_type {
            body.insert("secret_answer_type".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.short_link {
            body.insert("short_link".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.show_dob_date {
            body.insert("show_dob_date".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.show_dob_year {
            body.insert("show_dob_year".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.timezone {
            body.insert("timezone".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user_dob_day {
            body.insert("user_dob_day".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user_dob_month {
            body.insert("user_dob_month".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user_dob_year {
            body.insert("user_dob_year".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.user_title {
            body.insert("user_title".into(), serde_json::to_value(v).unwrap_or_default());
        }
        if let Some(v) = &params.username {
            body.insert("username".into(), serde_json::to_value(v).unwrap_or_default());
        }
        self.client.request(
            "put",
            &format!("/users/{user_id}"),
            None::<&[(&str, String)]>,
            Some(serde_json::Value::Object(body)),
        ).await
    }

    /// Get User Fields
    /// `GET /users/fields`
    pub async fn users_fields(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/users/fields",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Find Users
    /// `GET /users/find`
    pub async fn users_find(
        &self,
        username: Option<String>,
        custom_fields: Option<serde_json::Value>,
        fields_include: Option<Vec<String>>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &username {
            query.push(("username", v.to_string()));
        }
        if let Some(v) = &custom_fields {
            query.push(("custom_fields", v.to_string()));
        }
        if let Some(v) = &fields_include {
            for item in v {
                query.push(("fields_include", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/users/find",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Follow User
    /// `POST /users/{user_id}/followers`
    pub async fn users_follow(
        &self,
        user_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/users/{user_id}/followers"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get User Followers
    /// `GET /users/{user_id}/followers`
    pub async fn users_followers(
        &self,
        user_id: i64,
        order: Option<String>,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &order {
            query.push(("order", v.to_string()));
        }
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/users/{user_id}/followers"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Followed Users By User
    /// `GET /users/{user_id}/followings`
    pub async fn users_followings(
        &self,
        user_id: i64,
        order: Option<String>,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &order {
            query.push(("order", v.to_string()));
        }
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/users/{user_id}/followings"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get User
    /// `GET /users/{user_id}`
    pub async fn users_get(
        &self,
        user_id: i64,
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
            &format!("/users/{user_id}"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Ignore User
    /// `POST /users/{user_id}/ignore`
    pub async fn users_ignore(
        &self,
        user_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            &format!("/users/{user_id}/ignore"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Edit Ignoring Options
    /// `PUT /users/{user_id}/ignore`
    pub async fn users_ignore_edit(
        &self,
        user_id: i64,
        ignore_conversations: Option<bool>,
        ignore_content: Option<bool>,
        restrict_view_profile: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &ignore_conversations {
            query.push(("ignore_conversations", v.to_string()));
        }
        if let Some(v) = &ignore_content {
            query.push(("ignore_content", v.to_string()));
        }
        if let Some(v) = &restrict_view_profile {
            query.push(("restrict_view_profile", v.to_string()));
        }
        self.client.request(
            "put",
            &format!("/users/{user_id}/ignore"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Ignored Users
    /// `GET /users/ignored`
    pub async fn users_ignored(
        &self,
        total: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &total {
            query.push(("total", v.to_string()));
        }
        self.client.request(
            "get",
            "/users/ignored",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get User Likes
    /// `GET /users/{user_id}/likes`
    pub async fn users_likes(
        &self,
        user_id: i64,
        params: ForumUsersLikesParams,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.node_id {
            query.push(("node_id", v.to_string()));
        }
        if let Some(v) = &params.like_type {
            query.push(("like_type", v.to_string()));
        }
        if let Some(v) = &params.r#type {
            query.push(("type", v.to_string()));
        }
        if let Some(v) = &params.page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &params.content_type {
            query.push(("content_type", v.to_string()));
        }
        if let Some(v) = &params.search_user_id {
            query.push(("search_user_id", v.to_string()));
        }
        if let Some(v) = &params.stats {
            query.push(("stats", v.to_string()));
        }
        self.client.request(
            "get",
            &format!("/users/{user_id}/likes"),
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Get Users
    /// `GET /users`
    pub async fn users_list(
        &self,
        page: Option<i64>,
        limit: Option<i64>,
        fields_include: Option<Vec<String>>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        if let Some(v) = &fields_include {
            for item in v {
                query.push(("fields_include", item.to_string()));
            }
        }
        self.client.request(
            "get",
            "/users",
            Some(&query),
            None::<serde_json::Value>,
        ).await
    }

    /// Cancel Secret Answer Reset
    /// `DELETE /account/secret-answer/reset`
    pub async fn users_sa_cancel_reset(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            "/account/secret-answer/reset",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Reset Secret Answer
    /// `POST /account/secret-answer/reset`
    pub async fn users_sa_reset(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "post",
            "/account/secret-answer/reset",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Secret Answer Types
    /// `GET /users/secret-answer/types`
    pub async fn users_secret_answer_types(
        &self,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            "/users/secret-answer/types",
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Get Trophies
    /// `GET /users/{user_id}/trophies`
    pub async fn users_trophies(
        &self,
        user_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "get",
            &format!("/users/{user_id}/trophies"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Unfollow User
    /// `DELETE /users/{user_id}/followers`
    pub async fn users_unfollow(
        &self,
        user_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/users/{user_id}/followers"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

    /// Unignore User
    /// `DELETE /users/{user_id}/ignore`
    pub async fn users_unignore(
        &self,
        user_id: i64,
    ) -> Result<serde_json::Value> {
        self.client.request(
            "delete",
            &format!("/users/{user_id}/ignore"),
            None::<&[(&str, String)]>,
            None::<serde_json::Value>,
        ).await
    }

}
