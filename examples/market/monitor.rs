// cargo run --example market/monitor
//
// Мониторинг новых лотов по фильтрам (цена, категория).

use lolzteam::market::types::*;
use lolzteam::LolzteamClient;
use std::collections::HashSet;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: cargo run --example market/monitor -- YOUR_TOKEN");

    let client = LolzteamClient::new(&token);
    let market = client.expect("failed to build client").market();

    let mut seen_ids: HashSet<i64> = HashSet::new();

    println!("🔍 Мониторинг Steam аккаунтов (цена 10-100 руб)...\n");

    loop {
        match market
            .category_steam(MarketCategorySteamParams {
                pmin: Some(10),
                pmax: Some(100),
                order_by: Some("date_add".to_string()),
                page: Some(1),
                ..Default::default()
            })
            .await
        {
            Ok(resp) => {
                for item in resp.items.iter().take(20) {
                    if let (Some(item_id), Some(title), Some(price)) = (
                        item.get("item_id").and_then(|v| v.as_i64()),
                        item.get("title").and_then(|v| v.as_str()),
                        item.get("price").and_then(|v| v.as_i64()),
                    ) {
                        if !seen_ids.contains(&item_id) {
                            seen_ids.insert(item_id);
                            println!(
                                "🆕 [{}] {} - {} руб | https://lzt.market/{}",
                                item_id, title, price, item_id
                            );
                        }
                    }
                }
            }
            Err(e) => eprintln!("⚠️  err: {e}"),
        }

        sleep(Duration::from_secs(30)).await;
    }
}
