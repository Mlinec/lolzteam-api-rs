// cargo run --example forum/notifications -- YOUR_TOKEN

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: notifications <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.expect("failed to build client").forum();

    println!("--- уведомления ---");
    match forum.notifications_list(None, None, Some(10)).await {
        Ok(resp) => {
            println!("{} уведомлений", resp.notifications.len());
            for notif in resp.notifications.iter().take(10) {
                let status = if notif.notification_is_unread {
                    "NEW"
                } else {
                    "read"
                };
                println!(
                    "  [{}] {} - {} ({})",
                    notif.notification_id,
                    status,
                    notif.notification_type,
                    notif.notification_create_date
                );
            }
        }
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\ndone");
    Ok(())
}
