// cargo run --example forum_threads -- YOUR_TOKEN

use lolzteam::LolzteamClient;
use lolzteam::forum::types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: forum_threads <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.forum();

    println!("--- threads ---");
    let threads = forum
        .threads_list(ForumThreadsListParams {
            limit: Some(5),
            ..Default::default()
        })
        .await?;

    let thread_id;
    if let Some(arr) = threads.get("threads").and_then(|t| t.as_array()) {
        println!("{} threads", arr.len());
        for t in arr.iter().take(5) {
            println!(
                "  [{}] {} (views: {})",
                t["thread_id"],
                t["thread_title"].as_str().unwrap_or("?"),
                t["thread_view_count"]
            );
        }
        thread_id = arr.first().and_then(|t| t["thread_id"].as_i64()).unwrap_or(1);
    } else {
        thread_id = 1;
    }

    println!("\n--- thread #{thread_id} ---");
    match forum.threads_get(thread_id, None).await {
        Ok(thread) => {
            if let Some(t) = thread.get("thread") {
                println!("title: {}", t["thread_title"].as_str().unwrap_or("?"));
                println!("author: {}", t["creator_username"].as_str().unwrap_or("?"));
                println!("replies: {}", t["thread_reply_count"]);
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- navigation for #{thread_id} ---");
    match forum.threads_navigation(thread_id).await {
        Ok(nav) => println!(
            "keys: {:?}",
            nav.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- thread #{thread_id} followers ---");
    match forum.threads_followers(thread_id).await {
        Ok(f) => println!(
            "keys: {:?}",
            f.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
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
        Ok(recent) => {
            if let Some(arr) = recent.get("threads").and_then(|t| t.as_array()) {
                println!("found: {}", arr.len());
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- followed threads ---");
    match forum.threads_followed(None, None).await {
        Ok(followed) => {
            if let Some(arr) = followed.get("threads").and_then(|t| t.as_array()) {
                println!("count: {}", arr.len());
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    // посты в треде
    println!("\n--- posts in #{thread_id} ---");
    let posts = match forum
        .posts_list(ForumPostsListParams {
            thread_id: Some(thread_id),
            limit: Some(3),
            ..Default::default()
        })
        .await
    {
        Ok(posts) => {
            if let Some(arr) = posts.get("posts").and_then(|t| t.as_array()) {
                println!("{} posts", arr.len());
                for p in arr.iter().take(3) {
                    println!(
                        "  #{} by {}",
                        p["post_id"],
                        p["poster_username"].as_str().unwrap_or("?")
                    );
                }
            }
            Some(posts)
        }
        Err(e) => {
            eprintln!("  skip: {e}");
            None
        }
    };

    if let Some(post_id) = posts
        .as_ref()
        .and_then(|p| p.get("posts"))
        .and_then(|t| t.as_array())
        .and_then(|arr| arr.first())
        .and_then(|p| p["post_id"].as_i64())
    {
        println!("\n--- post #{post_id} ---");
        match forum.posts_get(post_id).await {
            Ok(post) => {
                if let Some(p) = post.get("post") {
                    println!(
                        "author: {}, likes: {}",
                        p["poster_username"].as_str().unwrap_or("?"),
                        p["post_like_count"]
                    );
                }
            }
            Err(e) => eprintln!("  skip: {e}"),
        }

        println!("\n--- likes on #{post_id} ---");
        match forum.posts_likes(post_id, None, None).await {
            Ok(likes) => println!(
                "keys: {:?}",
                likes.as_object().map(|m| m.keys().collect::<Vec<_>>())
            ),
            Err(e) => eprintln!("  skip: {e}"),
        }
    }

    println!("\ndone");
    Ok(())
}
