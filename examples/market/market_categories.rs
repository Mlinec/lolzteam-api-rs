// cargo run --example market_categories -- YOUR_TOKEN

use lolzteam::market::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: market_categories <TOKEN>");
    let client = LolzteamClient::new(&token);
    let market = client.market();

    println!("--- categories ---");
    match market.category_list(None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- all items (page 1) ---");
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

    println!("\n--- steam ---");
    match market
        .category_steam(MarketCategorySteamParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- fortnite ---");
    match market
        .category_fortnite(MarketCategoryFortniteParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- telegram ---");
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

    println!("\n--- discord ---");
    match market
        .category_discord(MarketCategoryDiscordParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- riot ---");
    match market
        .category_riot(MarketCategoryRiotParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- minecraft ---");
    match market
        .category_minecraft(MarketCategoryMinecraftParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- steam params ---");
    match market.category_params("steam".into()).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- steam games ---");
    match market.category_games("steam".into()).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- currency ---");
    match market.payments_currency().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\ndone");
    Ok(())
}
