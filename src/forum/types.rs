//! Auto-generated types for Forum API.
//! DO NOT EDIT — regenerate with `cargo run -p lolzteam-codegen`.

#![allow(unused, clippy::all)]

use serde::{Deserialize, Serialize};
use crate::models::*;

/// Parameters for `Conversations.Create` (POST /conversations).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumConversationsCreateParams {
    /// Allow members to delete their own messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_own_messages: Option<bool>,
    /// Allow edit messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_edit_messages: Option<bool>,
    /// Allow members to stick messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sticky_messages: Option<bool>,
    /// Is group. Set **false** if personal conversation, or set **true** if group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group: Option<bool>,
    /// First message. Required if **is_group**=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    /// Open invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_invite: Option<bool>,
    /// Id of recipient. Required if **is_group=false**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<i64>,
    /// List of recipients username's. Max recipients count is 10. Required if **is_group=true**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    /// The title of new conversation. Required if **is_group=1**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Parameters for `Conversations.Messages.List` (GET /conversations/{conversation_id}/messages).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumConversationsMessagesListParams {
    /// Page number of messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of messages in a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Ordering of messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Date to get older messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<i64>,
    /// Date to get newer messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<i64>,
}

/// Parameters for `Conversations.Update` (PUT /conversations).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumConversationsUpdateParams {
    /// Allow members to delete their own messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_own_messages: Option<bool>,
    /// Allow members to edit their own messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_edit_messages: Option<bool>,
    /// Allow members to stick messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sticky_messages: Option<bool>,
    /// Id of conversation.
    pub conversation_id: i64,
    /// Make conversation history visible to new members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_open: Option<bool>,
    /// Allow members to invite others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_invite: Option<bool>,
    /// New conversation title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Parameters for `Forums.Follow` (POST /forums/{forum_id}/followers).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumForumsFollowParams {
    /// Whether to receive notification as alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<bool>,
    /// Whether to receive notification as email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    /// Minimal contest amount. (Only for 766 forumId)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_contest_amount: Option<i64>,
    /// Whether to receive notification for post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<bool>,
    /// Prefix ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_ids: Option<Vec<i64>>,
}

/// Parameters for `Posts.List` (GET /posts).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumPostsListParams {
    /// Id of the containing thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i64>,
    /// Id of a post, posts that are in the same page with the specified post will be returned. **thread_id** may be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_of_post_id: Option<i64>,
    /// Page number of posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of posts in a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Ordering of posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// Parameters for `ProfilePosts.List` (GET /users/{user_id}/profile-posts).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumProfilePostsListParams {
    /// Filter to get only posts from the specified user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts_user_id: Option<i64>,
    /// Page number of contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of contents in a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of fields to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
}

/// Parameters for `Search.All` (POST /search).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumSearchAllParams {
    /// Id of the container forum to search for contents. Child forums of the specified forum will be included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    /// Number of results in a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Search query. Can be skipped if **user_id** is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Tag to search for tagged contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<serde_json::Value>,
}

/// Parameters for `Search.Posts` (POST /search/posts).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumSearchPostsParams {
    /// Number of post data to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    /// Id of the container forum to search for contents. Child forums of the specified forum will be included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    /// Number of results in a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Search query. Can be skipped if **user_id** is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Tag to search for tagged contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<serde_json::Value>,
}

/// Parameters for `Search.ProfilePosts` (POST /search/profile-posts).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumSearchProfilePostsParams {
    /// Number of results in a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Search query. Can be skipped if **user_id** is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// User ID to filter profile posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// Parameters for `Search.Tagged` (POST /search/tagged).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumSearchTaggedParams {
    /// Number of results in a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Tag to search for tagged contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Array of tags to search for tagged contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Parameters for `Search.Threads` (POST /search/threads).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumSearchThreadsParams {
    /// Number of thread data to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    /// Id of the container forum to search for contents. Child forums of the specified forum will be included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    /// Number of results in a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Search query. Can be skipped if **user_id** is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Tag to search for tagged contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<serde_json::Value>,
}

/// Parameters for `Threads.Claim` (POST /claims).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumThreadsClaimParams {
    /// Allow ask hidden content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_hidden_content: Option<bool>,
    /// Indicate the amount by which the responder deceived you.
    pub as_amount: f64,
    /// Contacts and wallets of the responder. Specify the known data about the responder, if any.
    /// Optional if **as_is_market_deal** is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_data: Option<String>,
    /// Funds transfer receipt.
    /// Upload a receipt for the transfer of funds, use the "View receipt" button in your wallet. May be uploaded to [Imgur](https://imgur.com/upload). Write "no" if you have not paid.
    /// Required if **as_is_market_deal** is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_funds_receipt: Option<String>,
    /// Did you buy account on the market?
    pub as_is_market_deal: bool,
    /// Market item id.
    /// Required if **as_is_market_deal** is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_market_item_id: Option<i64>,
    /// To whom the complaint is filed. Specify a nickname or a link to the profile.
    pub as_responder: String,
    /// Screenshot showing the respondent's Telegram login.
    /// If the correspondence was conducted in Telegram, upload a screenshot that will display the respondent's Telegram login against the background of your dialogue. The screenshot may be uploaded to [Imgur](https://imgur.com/upload). If the correspondence was conducted elsewhere, write "no".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_tg_login_screenshot: Option<String>,
    /// Allow commenting if user can't post in thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ignore_group: Option<bool>,
    /// Currency of Claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Don't alert followers about thread creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_alert_followers: Option<bool>,
    /// Hide contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_contacts: Option<bool>,
    /// Pay claim fee now or later. (Only for **transfer_type** = **notsafe**)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_claim: Option<String>,
    /// You should describe what's happened.
    /// - describe the situation in a nutshell. If you wish, you can describe the situation in more detail using the "Spoiler" function.
    /// - attach screenshots of correspondence. You may upload to the site [Imgur](https://imgur.com/upload) - for convenience, use Ctrl + V when uploading screenshots to the album.
    /// - other evidence;
    /// - notify the respondent about the complaint you created, familiarize him with hidden content
    /// 
    /// Describe the situation in as much detail as possible.
    pub post_body: String,
    /// Allow to reply only users with chosen or higher group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_group: Option<i64>,
    /// Date to schedule thread creation (format: `DD-MM-YYYY`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_date: Option<String>,
    /// Time to schedule thread creation (format: `HH:MM`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    /// Thread tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The transaction took place through a guarantor or there was a transfer to the market with a hold?
    /// Required if **as_is_market_deal** is 0.
    pub transfer_type: String,
    /// Receive forum notifications of new posts in this thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread: Option<bool>,
    /// Receive email notifications of new posts in this thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_email: Option<bool>,
    /// Watch thread state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_state: Option<bool>,
}

/// Parameters for `Threads.Create` (POST /threads).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumThreadsCreateParams {
    /// Allow ask hidden content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_hidden_content: Option<bool>,
    /// Allow commenting if user can't post in thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ignore_group: Option<bool>,
    /// Don't alert followers about thread creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_alert_followers: Option<bool>,
    /// Id of the target forum.
    pub forum_id: i64,
    /// Hide contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_contacts: Option<bool>,
    /// Content of the new thread.
    pub post_body: String,
    /// Prefix ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_id: Option<Vec<i64>>,
    /// Allow to reply only users with chosen or higher group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_group: Option<i64>,
    /// Date to schedule thread creation (format: `DD-MM-YYYY`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_date: Option<String>,
    /// Time to schedule thread creation (format: `HH:MM`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    /// Thread tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Thread title. Can be skipped if **title_en** set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Thread english title. Can be skipped if **title** set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
    /// Receive forum notifications of new posts in this thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread: Option<bool>,
    /// Receive email notifications of new posts in this thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_email: Option<bool>,
    /// Watch thread state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_state: Option<bool>,
}

/// Parameters for `Threads.CreateContest` (POST /contests).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumThreadsCreateContestParams {
    /// Allow ask hidden content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_hidden_content: Option<bool>,
    /// Allow commenting if user can't post in thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ignore_group: Option<bool>,
    /// Contest type.
    pub contest_type: String,
    /// Winner count (prize count). Optional if **prize_type** is **money**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_winners: Option<i64>,
    /// Don't alert followers about thread creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_alert_followers: Option<bool>,
    /// Hide contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_contacts: Option<bool>,
    /// Enable the distribution of money prizes by places. Optional if **prize_type** is **money**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_money_places: Option<bool>,
    /// Giveaway duration type. The maximum duration is 3 days. Required if **contest_type** is **by_finish_date**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_option: Option<String>,
    /// Giveaway duration value. The maximum duration is 3 days. Required if **contest_type** is **by_finish_date**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_value: Option<i64>,
    /// Content of the new contest.
    pub post_body: String,
    /// How much money will each winner receive. Optional if **prize_type** is **money**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_data_money: Option<f64>,
    /// How much money will receive each place. Required if **is_money_places** is **1**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_data_places: Option<Vec<f64>>,
    /// Which upgrade will each winner receive. Required if **prize_type** is **upgrades**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_data_upgrade: Option<i64>,
    /// Prize type.
    pub prize_type: String,
    /// Allow to reply only users with chosen or higher group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_group: Option<i64>,
    /// Sympathies for this week.
    pub require_like_count: i64,
    /// Sympathies for all time.
    pub require_total_like_count: i64,
    /// Date to schedule thread creation (format: `DD-MM-YYYY`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_date: Option<String>,
    /// Time to schedule thread creation (format: `HH:MM`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    /// Secret answer of your account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_answer: Option<String>,
    /// Thread tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Thread title. Can be skipped if **title_en** set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Thread english title. Can be skipped if **title** set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
    /// Receive forum notifications of new posts in this thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread: Option<bool>,
    /// Receive email notifications of new posts in this thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_email: Option<bool>,
    /// Watch thread state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_state: Option<bool>,
}

/// Parameters for `Threads.Edit` (PUT /threads/{thread_id}).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumThreadsEditParams {
    /// Allow ask hidden content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_hidden_content: Option<bool>,
    /// Allow commenting if user can't post in thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ignore_group: Option<bool>,
    /// Discussion state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_open: Option<bool>,
    /// Hide contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_contacts: Option<bool>,
    /// Prefix ids. Set "0" to remove all thread prefixes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_id: Option<Vec<i64>>,
    /// Allow to reply only users with chosen or higher group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_group: Option<i64>,
    /// Thread tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Thread title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Thread title english.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}

/// Parameters for `Threads.List` (GET /threads).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumThreadsListParams {
    /// Id of the containing forum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    /// Tab to get threads from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab: Option<String>,
    /// Thread state. Works only if **forum_id** is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter to get only threads created within the selected period. Works only if **forum_id** is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// Thread title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Search only in titles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_only: Option<bool>,
    /// Filter to get only threads created by the specified user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<i64>,
    /// Filter to get only sticky or non-sticky threads. By default, all threads will be included and sticky ones will be at the top of the result on the first page. In mixed mode, sticky threads are not counted towards **threads_total** and does not affect pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky: Option<bool>,
    /// Filter to get only threads with the specified prefix.
    #[serde(rename = "prefix_ids[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_ids: Option<Vec<i64>>,
    /// Filter to get only threads without the specified prefix.
    #[serde(rename = "prefix_ids_not[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_ids_not: Option<Vec<i64>>,
    /// Filter to get only threads with the specified tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_tag_id: Option<i64>,
    /// Page number of threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of threads in a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Ordering of threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Direction of threads ordering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Filter threads by creation date. Only works with 'thread_create_date' and 'thread_create_date_reverse' ordering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_create_date: Option<i64>,
    /// Filter threads by update date. Only works with 'thread_update_date' and 'thread_update_date_reverse' ordering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_update_date: Option<i64>,
    /// List of fields to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
}

/// Parameters for `Threads.Move` (POST /threads/{thread_id}/move).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumThreadsMoveParams {
    /// Apply thread prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_thread_prefix: Option<bool>,
    /// Forum id.
    pub node_id: String,
    /// Prefix ids. Set "0" to remove all thread prefixes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_id: Option<Vec<i64>>,
    /// Send a notification to users who are followed to target node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_alert: Option<bool>,
    /// Thread title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Thread title english.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}

/// Parameters for `Threads.Recent` (GET /threads/recent).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumThreadsRecentParams {
    /// Maximum number of days to search for threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Maximum number of result threads. The limit may get decreased if the value is too large.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Id of the container forum to search for threads. Child forums of the specified forum will be included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    /// Number of thread data to be returned. Default value is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
}

/// Parameters for `Users.Avatar.Upload` (POST /users/{user_id}/avatar).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumUsersAvatarUploadParams {
    /// Binary data of the avatar.
    pub avatar: String,
    /// Selection size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<i64>,
    /// The starting point of the selection by width. Default value - 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    /// The starting point of the selection by height. Default value - 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}

/// Parameters for `Users.Background.Upload` (POST /users/{user_id}/background).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumUsersBackgroundUploadParams {
    /// Binary data of the background. Background image must be 1920x1080 pixels
    pub background: String,
    /// Selection size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<i64>,
    /// The starting point of the selection by width. Default value - 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    /// The starting point of the selection by height. Default value - 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}

/// Parameters for `Users.Edit` (PUT /users/{user_id}).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumUsersEditParams {
    /// Whether user activity is visible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_visible: Option<bool>,
    /// Alert settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<serde_json::Value>,
    /// Who can invite you to groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_invite_group: Option<String>,
    /// Who can post on your profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_post_profile: Option<String>,
    /// Who can see your news feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_receive_news_feed: Option<String>,
    /// Who can send you personal conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_send_personal_conversation: Option<String>,
    /// Who can view your profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_view_profile: Option<String>,
    /// This message is shown when someone wants to write to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conv_welcome_message: Option<String>,
    /// Id of the banner you want to display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_banner_id: Option<i64>,
    /// Id of the group you want to display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_group_id: Option<i64>,
    /// Id of the icon group you want to display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_icon_group_id: Option<i64>,
    /// Custom user profile fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    /// User gender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Hide username change logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_username_change_logs: Option<bool>,
    /// User interface language ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<i64>,
    /// Whether to receive admin emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_admin_email: Option<bool>,
    /// Secret answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_answer: Option<String>,
    /// Secret answer type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_answer_type: Option<i64>,
    /// Profile short link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_link: Option<String>,
    /// Show date of birth (day and month).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_dob_date: Option<bool>,
    /// Show year of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_dob_year: Option<bool>,
    /// User timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Your date of birth (day).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_dob_day: Option<i64>,
    /// Your date of birth (month).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_dob_month: Option<i64>,
    /// Your date of birth (year).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_dob_year: Option<i64>,
    /// New custom title of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    /// New username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Parameters for `Users.Likes` (GET /users/{user_id}/likes).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumUsersLikesParams {
    /// Filter by forum section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<i64>,
    /// Like type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_type: Option<String>,
    /// Likes type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Get only likes from specified user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_user_id: Option<i64>,
    /// Show weekly statistics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<bool>,
}

