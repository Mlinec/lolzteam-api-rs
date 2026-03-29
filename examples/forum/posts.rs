// cargo run --example forum/posts -- YOUR_TOKEN

use lolzteam::forum::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: posts <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.expect("failed to build client").forum();

    println!("--- последние посты ---");
    let posts = forum
        .posts_list(ForumPostsListParams {
            limit: Some(5),
            ..Default::default()
        })
        .await?;

    println!("{} постов", posts.posts.len());
    for post in posts.posts.iter().take(5) {
        println!("  [пост #{}] от {}", post.post_id, post.poster_username);
    }

    if let Some(_post) = posts.posts.first() {
        let thread_id = posts.thread.thread_id;

        println!("\n--- посты в треде #{thread_id} ---");
        match forum
            .posts_list(ForumPostsListParams {
                thread_id: Some(thread_id),
                limit: Some(3),
                ..Default::default()
            })
            .await
        {
            Ok(resp) => {
                println!("тред: {}", resp.thread.thread_title);
                println!("автор: {}", resp.thread.creator_username);
            }
            Err(e) => eprintln!("  skip: {e}"),
        }

        println!("\n--- лайки треда #{thread_id} ---");
        match forum.threads_get(thread_id, None).await {
            Ok(resp) => {
                println!("просмотров: {}", resp.thread.thread_view_count);
                println!("постов: {}", resp.thread.thread_post_count);
            }
            Err(e) => eprintln!("  skip: {e}"),
        }
    }

    println!("\ndone");
    Ok(())
}
