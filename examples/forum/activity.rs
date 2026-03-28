// cargo run --example forum/activity
//
// Активность пользователя (посты, треды, репутация).

use lolzteam::forum::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: cargo run --example forum/activity -- YOUR_TOKEN [USER_ID]");

    let user_id: i64 = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "1".to_string())
        .parse()
        .expect("USER_ID must be a number");

    let client = LolzteamClient::new(&token);
    let forum = client.expect("failed to build client").forum();

    println!("📊 Активность пользователя #{user_id}\n");

    println!("--- профиль ---");
    match forum.users_get(user_id, None).await {
        Ok(resp) => {
            let user = &resp.user;
            println!("username: {}", user.username);
            println!("регистрация: {}", user.user_register_date);
            println!("сообщений: {}", user.user_message_count);
            println!("лайков: {}", user.user_like_count);
        }
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\n--- последние посты ---");
    match forum
        .search_posts(ForumSearchPostsParams {
            user_id: Some(user_id.to_string()),
            limit: Some(5),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => {
            println!("{} постов", resp.data.len());
            for post in resp.data.iter().take(5) {
                if let (Some(post_id), Some(thread_id), Some(likes)) = (
                    post.get("post_id").and_then(|v| v.as_i64()),
                    post.get("thread_id").and_then(|v| v.as_i64()),
                    post.get("post_like_count").and_then(|v| v.as_i64()),
                ) {
                    println!("  [{}] в треде #{} - {} лайков", post_id, thread_id, likes);
                }
            }
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- созданные треды ---");
    match forum
        .threads_list(ForumThreadsListParams {
            creator_user_id: Some(user_id),
            limit: Some(5),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => {
            println!("{} тредов", resp.threads.len());
            for thread in resp.threads.iter().take(5) {
                println!(
                    "  [{}] {} - {} просмотров",
                    thread.thread_id, thread.thread_title, thread.thread_view_count
                );
            }
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- посты на стене ---");
    match forum
        .profile_posts_list(
            user_id,
            ForumProfilePostsListParams {
                posts_user_id: None,
                limit: Some(5),
                ..Default::default()
            },
        )
        .await
    {
        Ok(resp) => {
            println!("{} постов на стене", resp.profile_posts.len());
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- подписчики ---");
    match forum.users_followers(user_id, None, None, None).await {
        Ok(resp) => {
            println!("{} подписчиков", resp.users.len());
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- подписки ---");
    match forum.users_followings(user_id, None, None, None).await {
        Ok(resp) => {
            println!("{} подписок", resp.users.len());
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
