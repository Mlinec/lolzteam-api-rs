// cargo run --example forum_threads -- YOUR_TOKEN

use lolzteam::forum::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: forum_threads <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.expect("failed to build client").forum();

    println!("--- threads ---");
    let threads = forum
        .threads_list(ForumThreadsListParams {
            limit: Some(5),
            ..Default::default()
        })
        .await?;

    println!("{} threads", threads.threads.len());
    for t in threads.threads.iter().take(5) {
        println!(
            "  [{}] {} (views: {})",
            t.thread_id, t.thread_title, t.thread_view_count
        );
    }

    let thread_id = threads.threads.first().map(|t| t.thread_id).unwrap_or(1);

    println!("\n--- thread #{thread_id} ---");
    match forum.threads_get(thread_id, None).await {
        Ok(resp) => {
            println!("title: {}", resp.thread.thread_title);
            println!("author: {}", resp.thread.creator_username);
            println!("posts: {}", resp.thread.thread_post_count);
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- navigation for #{thread_id} ---");
    match forum.threads_navigation(thread_id).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- thread #{thread_id} followers ---");
    match forum.threads_followers(thread_id).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- recent (7d) ---");
    match forum
        .threads_recent(ForumThreadsRecentParams {
            days: Some(7),
            limit: Some(3),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("{} threads", resp.threads.len()),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- followed threads ---");
    match forum.threads_followed(None, None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- posts in #{thread_id} ---");
    match forum
        .posts_list(ForumPostsListParams {
            thread_id: Some(thread_id),
            limit: Some(3),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => {
            println!("{} posts", resp.posts.len());
            for p in resp.posts.iter().take(3) {
                println!("  post #{} by {}", p.post_id, p.poster_username);
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
