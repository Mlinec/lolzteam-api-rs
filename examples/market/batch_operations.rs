// cargo run --example market/batch_operations
//
// Пакетные операции (массовый bump, изменение цен).

use lolzteam::market::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: cargo run --example market/batch_operations -- YOUR_TOKEN");

    let client = LolzteamClient::new(&token);
    let market = client.expect("failed to build client").market();

    println!("📦 Пакетные операции\n");

    println!("--- получение своих лотов ---");
    let items = market
        .list_user(MarketListUserParams {
            user_id: None, // текущий пользователь
            show: Some("active".to_string()),
            ..Default::default()
        })
        .await?;

    println!("{} активных лотов\n", items.items.len());

    if items.items.is_empty() {
        println!("Нет активных лотов для операций");
        return Ok(());
    }

    // Массовый bump (поднятие лотов)
    println!("--- массовый bump ---");
    let mut bumped = 0;
    let mut skipped = 0;

    for item in items.items.iter().take(5) {
        match market.managing_bump(item.item_id.unwrap_or(0)).await {
            Ok(_) => {
                let title = item.title.as_deref().unwrap_or("N/A");
                println!("✅ [{}] {} - поднят", item.item_id.unwrap_or(0), title);
                bumped += 1;
            }
            Err(e) => {
                let title = item.title.as_deref().unwrap_or("N/A");
                println!(
                    "⏭️  [{}] {} - пропущен: {}",
                    item.item_id.unwrap_or(0),
                    title,
                    e
                );
                skipped += 1;
            }
        }
    }

    println!("\nИтого: поднято {}, пропущено {}", bumped, skipped);

    // Статистика по лотам
    println!("\n--- статистика ---");
    let mut total_price: i64 = 0;
    let mut total_views: i64 = 0;
    for item in items.items.iter().take(10) {
        total_price += item.price.unwrap_or(0);
        total_views += item.view_count.unwrap_or(0);
    }
    println!("Общая стоимость первых 10 лотов: {} руб", total_price);
    println!("Общее количество просмотров: {}", total_views);

    println!(
        "\nНОТА: массовое изменение цен не выполнено (используй managing_edit для каждого лота)"
    );
    println!("done");
    Ok(())
}
