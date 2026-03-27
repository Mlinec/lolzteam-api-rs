// cargo run --example forum/conversations -- YOUR_TOKEN

use lolzteam::forum::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: conversations <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.expect("failed to build client").forum();

    println!("--- список диалогов ---");
    let conversations = forum.conversations_list(None, None, Some(5)).await?;

    println!("{} диалогов", conversations.conversations.len());
    for conv in conversations.conversations.iter().take(5) {
        println!(
            "  [{}] {} (сообщений: {})",
            conv.conversation_id, conv.conversation_title, conv.conversation_message_count
        );
    }

    if let Some(conv) = conversations.conversations.first() {
        let conv_id = conv.conversation_id;

        println!("\n--- диалог #{conv_id} ---");
        match forum.conversations_get(conv_id).await {
            Ok(resp) => {
                println!("title: {}", resp.conversation.conversation_title);
                println!("участников: {}", resp.conversation.recipients.len());
            }
            Err(e) => eprintln!("  skip: {e}"),
        }

        println!("\n--- сообщения в #{conv_id} ---");
        match forum
            .conversations_messages_list(
                conv_id,
                ForumConversationsMessagesListParams {
                    limit: Some(3),
                    ..Default::default()
                },
            )
            .await
        {
            Ok(resp) => {
                println!("{} сообщений", resp.messages.len());
                for msg in resp.messages.iter().take(3) {
                    println!("  [{}] от {}", msg.message_id, msg.creator_username);
                }
            }
            Err(e) => eprintln!("  skip: {e}"),
        }
    }

    println!("\ndone");
    Ok(())
}
