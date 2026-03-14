// cargo run --example forum_misc -- YOUR_TOKEN
//
// уведомления, переписки, чатбокс, теги и т.д.

use lolzteam::LolzteamClient;
use lolzteam::forum::types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: forum_misc <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.forum();

    println!("--- notifications ---");
    match forum.notifications_list(None, None, Some(5)).await {
        Ok(notifs) => {
            if let Some(arr) = notifs.get("notifications").and_then(|n| n.as_array()) {
                println!("{} notifications", arr.len());
                for n in arr.iter().take(3) {
                    println!(
                        "  [{}] {}/{}",
                        n["notification_id"],
                        n["content_type"].as_str().unwrap_or("?"),
                        n["content_action"].as_str().unwrap_or("?")
                    );
                }
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- conversations ---");
    match forum.conversations_list(None, None, Some(5)).await {
        Ok(convs) => {
            if let Some(arr) = convs.get("conversations").and_then(|c| c.as_array()) {
                println!("{} conversations", arr.len());
                for c in arr.iter().take(3) {
                    println!(
                        "  [{}] {} ({} msgs)",
                        c["conversation_id"],
                        c["conversation_title"].as_str().unwrap_or("?"),
                        c["conversation_message_count"]
                    );
                }
            }

            // если есть хоть одна — подгрузим детали
            if let Some(conv_id) = convs
                .get("conversations")
                .and_then(|c| c.as_array())
                .and_then(|arr| arr.first())
                .and_then(|c| c["conversation_id"].as_i64())
            {
                println!("\n--- conversation #{conv_id} ---");
                match forum.conversations_get(conv_id).await {
                    Ok(conv) => {
                        if let Some(c) = conv.get("conversation") {
                            println!(
                                "{} by {}",
                                c["conversation_title"].as_str().unwrap_or("?"),
                                c["creator_username"].as_str().unwrap_or("?")
                            );
                        }
                    }
                    Err(e) => eprintln!("  skip: {e}"),
                }

                println!("\n--- messages in #{conv_id} ---");
                match forum
                    .conversations_messages_list(
                        conv_id,
                        ForumConversationsMessagesListParams::default(),
                    )
                    .await
                {
                    Ok(msgs) => {
                        if let Some(arr) = msgs.get("messages").and_then(|m| m.as_array()) {
                            println!("{} msgs", arr.len());
                            for m in arr.iter().take(3) {
                                println!(
                                    "  [{}/{}] by {}",
                                    m["message_id"],
                                    m["conversation_id"],
                                    m["creator_username"].as_str().unwrap_or("?")
                                );
                            }
                        }
                    }
                    Err(e) => eprintln!("  skip: {e}"),
                }
            }
        }
        Err(e) => eprintln!("  skip (может быть 403): {e}"),
    }

    println!("\n--- chatbox ---");
    match forum.chatbox_index(None).await {
        Ok(cb) => println!(
            "keys: {:?}",
            cb.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- chatbox leaderboard ---");
    match forum.chatbox_get_leaderboard(None).await {
        Ok(lb) => println!(
            "keys: {:?}",
            lb.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- popular tags ---");
    match forum.tags_popular().await {
        Ok(tags) => {
            if let Some(arr) = tags.get("tags").and_then(|t| t.as_array()) {
                println!("{} tags", arr.len());
                for t in arr.iter().take(5) {
                    println!(
                        "  {} ({})",
                        t["tag_text"].as_str().unwrap_or("?"),
                        t["tag_use_count"]
                    );
                }
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- tags search: cs ---");
    match forum.tags_find("cs".into()).await {
        Ok(found) => println!(
            "keys: {:?}",
            found.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- forms ---");
    match forum.forms_list(None).await {
        Ok(forms) => println!(
            "keys: {:?}",
            forms.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- feed options ---");
    match forum.forums_get_feed_options().await {
        Ok(feed) => println!(
            "keys: {:?}",
            feed.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- ignored users ---");
    match forum.users_ignored(None).await {
        Ok(ignored) => println!(
            "keys: {:?}",
            ignored.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- my likes ---");
    match forum
        .users_likes(5285311, ForumUsersLikesParams::default())
        .await
    {
        Ok(likes) => println!(
            "keys: {:?}",
            likes.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
