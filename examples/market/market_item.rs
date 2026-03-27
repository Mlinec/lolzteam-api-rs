// cargo run --example market_item -- YOUR_TOKEN ITEM_ID
//
// Работа с конкретным аккаунтом на маркете.

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: market_item <TOKEN> <ITEM_ID>");
    let item_id: i64 = std::env::args()
        .nth(2)
        .expect("Usage: market_item <TOKEN> <ITEM_ID>")
        .parse()
        .expect("ITEM_ID must be a number");

    let client = LolzteamClient::new(&token);
    let market = client.expect("failed to build client").market();

    // Получить информацию об аккаунте
    println!("--- item #{item_id} ---");
    match market.managing_get(item_id, None).await {
        Ok(resp) => {
            let item = &resp.item;
            println!("title: {}", item.title);
            println!("price: {}", item.price);
            println!("category: {}", item.category_id);
        }
        Err(e) => eprintln!("  err: {e}"),
    }

    // AI-оценка цены
    println!("\n--- AI price ---");
    match market.managing_ai_price(item_id).await {
        Ok(resp) => println!("ai price: {}", resp.price),
        Err(e) => eprintln!("  skip: {e}"),
    }

    // Проверить гарантию
    println!("\n--- guarantee check ---");
    match market.managing_check_guarantee(item_id).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
