// cargo run --example market_search -- YOUR_TOKEN
//
// Поиск аккаунтов на маркете с фильтрами.

use lolzteam::market::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: market_search <TOKEN>");
    let client = LolzteamClient::new(&token);
    let market = client.market();

    // Steam аккаунты с фильтром по цене
    println!("--- steam: 10-50 RUB ---");
    match market
        .category_steam(MarketCategorySteamParams {
            pmin: Some(10),
            pmax: Some(50),
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    // Telegram аккаунты
    println!("\n--- telegram: page 1 ---");
    match market
        .category_telegram(MarketCategoryTelegramParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    // Discord аккаунты с фильтром по цене
    println!("\n--- discord: 5-100 RUB ---");
    match market
        .category_discord(MarketCategoryDiscordParams {
            pmin: Some(5),
            pmax: Some(100),
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    // VPN аккаунты
    println!("\n--- vpn ---");
    match market
        .category_vpn(MarketCategoryVpnParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    // ChatGPT аккаунты
    println!("\n--- chatgpt ---");
    match market
        .category_chat_gpt(MarketCategoryChatGptParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    // Параметры категории (какие фильтры доступны)
    println!("\n--- steam category params ---");
    match market.category_params("steam".into()).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    // Поиск по всем категориям
    println!("\n--- all categories: page 1 ---");
    match market
        .category_all(MarketCategoryAllParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\ndone");
    Ok(())
}
