// cargo run --example market/orders -- YOUR_TOKEN

use lolzteam::market::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: orders <TOKEN>");
    let client = LolzteamClient::new(&token);
    let market = client.expect("failed to build client").market();

    println!("--- мои заказы ---");
    match market
        .list_orders(MarketListOrdersParams {
            show: Some("active".to_string()),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => {
            println!("{} заказов", resp.items.len());
            for item in resp.items.iter().take(10) {
                let title = item.title.as_deref().unwrap_or("N/A");
                let price = item.price.unwrap_or(0);
                let item_id = item.item_id.unwrap_or(0);
                let state = item.item_state.as_deref().unwrap_or("N/A");
                println!(
                    "  [{}] {} - {} руб (статус: {})",
                    item_id, title, price, state
                );
            }
        }
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\n--- список категорий для заказов ---");
    match market.category_list(None).await {
        Ok(resp) => {
            if let Some(categories) = resp.category.as_object() {
                println!("{} категорий", categories.len());
                for (id, cat) in categories.iter().take(5) {
                    if let Some(name) = cat.get("category_name").and_then(|v| v.as_str()) {
                        println!("  [{}] {}", id, name);
                    }
                }
            }
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\nНОТА: создание заказа не выполнено (используй publishing_add для создания)");
    println!("done");
    Ok(())
}
