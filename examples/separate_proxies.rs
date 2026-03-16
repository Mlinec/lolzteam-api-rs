// cargo run --example separate_proxies -- YOUR_TOKEN
//
// Раздельные прокси для форума и маркета + кастомные настройки.

use lolzteam::LolzteamClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: separate_proxies <TOKEN>");

    // Раздельные прокси: форум через HTTP, маркет через SOCKS5
    let client = LolzteamClient::builder(&token)
        .forum_proxy("http://user:pass@proxy1.example.com:8080")
        .market_proxy("socks5://proxy2.example.com:1080")
        .max_retries(3)
        .timeout(Duration::from_secs(15))
        .build()?;

    println!("--- forum via HTTP proxy ---");
    match client.forum().users_get(1, None).await {
        Ok(resp) => println!("user: {} (id: {})", resp.user.username, resp.user.user_id),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- market via SOCKS5 proxy ---");
    match client.market().profile_get(None).await {
        Ok(resp) => println!("user: {} (id: {})", resp.user.username, resp.user.user_id),
        Err(e) => eprintln!("  err: {e}"),
    }

    // Можно также задать кастомные base URL (для тестового окружения)
    let _test_client = LolzteamClient::builder(&token)
        .forum_base_url("https://test-api.lolz.live")
        .market_base_url("https://test-api.lzt.market")
        .build()?;

    println!("\ndone");
    Ok(())
}
