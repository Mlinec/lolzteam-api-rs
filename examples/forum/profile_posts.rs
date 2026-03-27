// cargo run --example forum/profile_posts -- YOUR_TOKEN USER_ID

use lolzteam::forum::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: profile_posts <TOKEN> <USER_ID>");
    let user_id: i64 = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "1".to_string())
        .parse()
        .expect("USER_ID must be a number");

    let client = LolzteamClient::new(&token);
    let forum = client.forum();

    println!("--- посты на стене пользователя #{user_id} ---");
    let posts = forum
        .profile_posts_list(
            user_id,
            ForumProfilePostsListParams {
                posts_user_id: None,
                limit: Some(10),
                ..Default::default()
            },
        )
        .await?;

    println!("{} постов", posts.profile_posts.len());
    for post in posts.profile_posts.iter() {
        println!(
            "  [{}] от {} - {}",
            post.profile_post_id,
            post.poster_username,
            &post.post_body_plain_text[..post.post_body_plain_text.len().min(50)]
        );
    }

    if let Some(post) = posts.profile_posts.first() {
        let post_id = post.profile_post_id;

        println!("\n--- пост #{post_id} ---");
        match forum.profile_posts_get(post_id).await {
            Ok(resp) => {
                println!("автор: {}", resp.profile_post.poster_username);
                println!("timeline: {}", resp.profile_post.timeline_username);
                println!("лайков: {}", resp.profile_post.post_like_count);
            }
            Err(e) => eprintln!("  skip: {e}"),
        }

        println!("\n--- комментарии к #{post_id} ---");
        match forum
            .profile_posts_comments_list(post_id, None, Some(5))
            .await
        {
            Ok(resp) => {
                println!("{} комментариев", resp.comments.len());
                for comment in resp.comments.iter().take(5) {
                    println!("  [{}] от {}", comment.comment_id, comment.comment_username);
                }
            }
            Err(e) => eprintln!("  skip: {e}"),
        }
    }

    println!("\ndone");
    Ok(())
}
