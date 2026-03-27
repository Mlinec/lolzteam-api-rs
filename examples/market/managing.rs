// cargo run --example market/managing -- YOUR_TOKEN ITEM_ID
//
// Управление своим лотом на маркете.

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: managing <TOKEN> <ITEM_ID>");
    let item_id: i64 = std::env::args()
        .nth(2)
        .expect("Usage: managing <TOKEN> <ITEM_ID>")
        .parse()
        .expect("ITEM_ID must be a number");

    let client = LolzteamClient::new(&token);
    let market = client.expect("failed to build client").market();

    println!("--- информация о лоте #{item_id} ---");
    match market.managing_get(item_id, None).await {
        Ok(resp) => {
            let item = &resp.item;
            println!("title: {}", item.title);
            println!("price: {} руб", item.price);
            println!("category: {}", item.category_id);
            println!("views: {}", item.view_count);
        }
        Err(e) => {
            eprintln!("err: {e}");
            return Ok(());
        }
    }

    println!("\n--- AI-оценка цены ---");
    match market.managing_ai_price(item_id).await {
        Ok(resp) => println!("рекомендуемая цена: {} руб", resp.price),
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- проверка гарантии ---");
    match market.managing_check_guarantee(item_id).await {
        Ok(resp) => println!("гарантия: {:#?}", resp),
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- добавить в избранное ---");
    match market.managing_favorite(item_id).await {
        Ok(resp) => println!("добавлено: {:#?}", resp),
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- удалить из избранного ---");
    match market.managing_unfavorite(item_id).await {
        Ok(resp) => println!("удалено: {:#?}", resp),
        Err(e) => eprintln!("skip: {e}"),
    }

    println!("\n--- поднять лот (bump) ---");
    match market.managing_bump(item_id).await {
        Ok(resp) => println!("поднят: {:#?}", resp),
        Err(e) => eprintln!("skip (возможно, слишком рано): {e}"),
    }

    println!("\nНОТА: изменение цены/описания не выполнено (используй managing_edit)");
    println!("done");
    Ok(())
}
