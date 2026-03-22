// cargo run --example market/arbitrage -- YOUR_TOKEN

use lolzteam::market::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: arbitrage <TOKEN>");
    let client = LolzteamClient::new(&token);
    let market = client.market();

    println!("--- мои споры (как покупатель) ---");
    match market
        .list_orders(MarketListOrdersParams {
            show: Some("paid".to_string()),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => {
            println!("{} оплаченных заказов", resp.items.len());
            for item in resp.items.iter().take(5) {
                let title = item.title.as_deref().unwrap_or("N/A");
                let price = item.price.unwrap_or(0);
                let item_id = item.item_id.unwrap_or(0);
                println!("  [{}] {} - {} руб", item_id, title, price);
            }
        }
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\n--- мои продажи (как продавец) ---");
    match market
        .list_user(MarketListUserParams {
            user_id: None, // текущий пользователь
            show: Some("paid".to_string()),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => {
            println!("{} проданных аккаунтов", resp.items.len());
            for item in resp.items.iter().take(5) {
                let title = item.title.as_deref().unwrap_or("N/A");
                let price = item.price.unwrap_or(0);
                let item_id = item.item_id.unwrap_or(0);
                println!("  [{}] {} - {} руб", item_id, title, price);
            }
        }
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\nНОТА: создание спора не выполнено (используй managing_create_claim для создания)");
    println!("done");
    Ok(())
}
