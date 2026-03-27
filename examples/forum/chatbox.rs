// cargo run --example forum/chatbox -- YOUR_TOKEN

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: chatbox <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.expect("failed to build client").forum();

    println!("--- chatbox index ---");
    match forum.chatbox_index(None).await {
        Ok(resp) => {
            println!("комнат: {}", resp.rooms.len());
            for room in resp.rooms.iter().take(3) {
                if let Some(name) = room.get("room_name").and_then(|v| v.as_str()) {
                    println!("  комната: {}", name);
                }
            }
        }
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\n--- chatbox messages ---");
    match forum
        .chatbox_get_messages(serde_json::json!("general"), None)
        .await
    {
        Ok(resp) => {
            println!("{} сообщений", resp.messages.len());
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- chatbox online ---");
    match forum.chatbox_online(serde_json::json!(null)).await {
        Ok(resp) => {
            println!("{} онлайн", resp.users.len());
            for user in resp.users.iter().take(10) {
                if let (Some(username), Some(user_id)) = (
                    user.get("username").and_then(|v| v.as_str()),
                    user.get("user_id").and_then(|v| v.as_i64()),
                ) {
                    println!("  {} (id: {})", username, user_id);
                }
            }
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- chatbox leaderboard ---");
    match forum.chatbox_get_leaderboard(None).await {
        Ok(resp) => {
            println!("{} пользователей в топе", resp.leaderboard.len());
            for user in resp.leaderboard.iter().take(5) {
                if let (Some(username), Some(msg_count)) = (
                    user.get("username").and_then(|v| v.as_str()),
                    user.get("message_count").and_then(|v| v.as_i64()),
                ) {
                    println!("  {} - {} сообщений", username, msg_count);
                }
            }
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- ignore list ---");
    match forum.chatbox_get_ignore().await {
        Ok(resp) => {
            println!("{} в игноре", resp.ignored.len());
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
