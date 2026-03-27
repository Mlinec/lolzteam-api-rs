// cargo run --example market/purchasing -- YOUR_TOKEN ITEM_ID
//
// Покупка аккаунта (только проверка, без реальной покупки).

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: purchasing <TOKEN> <ITEM_ID>");
    let item_id: i64 = std::env::args()
        .nth(2)
        .expect("Usage: purchasing <TOKEN> <ITEM_ID>")
        .parse()
        .expect("ITEM_ID must be a number");

    let client = LolzteamClient::new(&token);
    let market = client.expect("failed to build client").market();

    println!("--- информация об аккаунте #{item_id} ---");
    match market.managing_get(item_id, None).await {
        Ok(resp) => {
            let item = &resp.item;
            println!("title: {}", item.title);
            println!("price: {} руб", item.price);
            println!("seller: {}", item.seller);
        }
        Err(e) => {
            eprintln!("err: {e}");
            return Ok(());
        }
    }

    println!("\n--- проверка возможности покупки ---");
    match market.purchasing_check(item_id).await {
        Ok(resp) => println!("можно купить: {:#?}", resp),
        Err(e) => {
            eprintln!("нельзя купить: {e}");
            return Ok(());
        }
    }

    println!("\n--- запрос скидки ---");
    match market
        .purchasing_discount_request(item_id, 10.0, None)
        .await
    {
        Ok(resp) => println!("скидка запрошена: {:#?}", resp),
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- отмена запроса скидки ---");
    match market.purchasing_discount_cancel(item_id).await {
        Ok(resp) => println!("скидка отменена: {:#?}", resp),
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\nНОТА: реальная покупка не выполнена (используй purchasing_fast_buy для покупки)");
    println!("done");
    Ok(())
}
