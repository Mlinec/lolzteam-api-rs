// cargo run --example forum_misc -- YOUR_TOKEN
//
// уведомления, переписки, чатбокс, теги и т.д.

use lolzteam::forum::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: forum_misc <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.expect("failed to build client").forum();

    println!("--- notifications ---");
    match forum.notifications_list(None, None, Some(5)).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- conversations ---");
    match forum.conversations_list(None, None, Some(5)).await {
        Ok(resp) => {
            println!("{} conversations", resp.conversations.len());

            if let Some(first) = resp.conversations.first() {
                let conv_id = first.conversation_id;

                println!("\n--- conversation #{conv_id} ---");
                match forum.conversations_get(conv_id).await {
                    Ok(conv) => println!("{:#?}", conv),
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
                    Ok(msgs) => println!("{} msgs", msgs.messages.len()),
                    Err(e) => eprintln!("  skip: {e}"),
                }
            }
        }
        Err(e) => eprintln!("  skip (может быть 403): {e}"),
    }

    println!("\n--- chatbox ---");
    match forum.chatbox_index(None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- chatbox leaderboard ---");
    match forum.chatbox_get_leaderboard(None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- popular tags ---");
    match forum.tags_popular().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- tags search: cs ---");
    match forum.tags_find("cs".into()).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- forms ---");
    match forum.forms_list(None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- feed options ---");
    match forum.forums_get_feed_options().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- ignored users ---");
    match forum.users_ignored(None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- my likes ---");
    match forum
        .users_likes(5285311, ForumUsersLikesParams::default())
        .await
    {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
