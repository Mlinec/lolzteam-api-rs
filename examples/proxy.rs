// cargo run --example proxy -- YOUR_TOKEN socks5://127.0.0.1:1080

use lolzteam::LolzteamClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: proxy <TOKEN> <PROXY_URL>");
    let proxy_url = std::env::args()
        .nth(2)
        .expect("Usage: proxy <TOKEN> <PROXY_URL>");

    let client = LolzteamClient::builder(&token)
        .proxy(&proxy_url)
        .max_retries(3)
        .timeout(Duration::from_secs(15))
        .build()?;

    println!("proxy: {proxy_url}");

    match client.forum().users_get(1, None).await {
        Ok(data) => println!("{:#?}", data),
        Err(e) => eprintln!("err: {e}"),
    }

    Ok(())
}
