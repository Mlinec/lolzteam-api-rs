// cargo run --example market_categories -- YOUR_TOKEN

use lolzteam::LolzteamClient;
use lolzteam::market::types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: market_categories <TOKEN>");
    let client = LolzteamClient::new(&token);
    let market = client.market();

    println!("--- categories ---");
    match market.category_list(None).await {
        Ok(cats) => println!(
            "keys: {:?}",
            cats.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
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
        Ok(all) => {
            if let Some(arr) = all.get("items").and_then(|i| i.as_array()) {
                println!("{} items", arr.len());
                for item in arr.iter().take(3) {
                    println!(
                        "  [{}] price={} cat={}",
                        item["item_id"], item["price"], item["category_id"]
                    );
                }
            }
        }
        Err(e) => eprintln!("  err: {e}"),
    }

    // отдельные категории
    println!("\n--- steam ---");
    match market
        .category_steam(MarketCategorySteamParams {
            page: Some(1),
            ..Default::default()
        })
        .await
    {
        Ok(steam) => {
            if let Some(arr) = steam.get("items").and_then(|i| i.as_array()) {
                println!("{} items", arr.len());
                for item in arr.iter().take(3) {
                    println!(
                        "  [{}] price={} state={}",
                        item["item_id"],
                        item["price"],
                        item["item_state"].as_str().unwrap_or("?")
                    );
                }
            }
        }
        Err(e) => eprintln!("  err: {e}"),
    }

    for (name, result) in [
        (
            "fortnite",
            market
                .category_fortnite(MarketCategoryFortniteParams {
                    page: Some(1),
                    ..Default::default()
                })
                .await,
        ),
        (
            "telegram",
            market
                .category_telegram(MarketCategoryTelegramParams {
                    page: Some(1),
                    ..Default::default()
                })
                .await,
        ),
        (
            "discord",
            market
                .category_discord(MarketCategoryDiscordParams {
                    page: Some(1),
                    ..Default::default()
                })
                .await,
        ),
        (
            "riot",
            market
                .category_riot(MarketCategoryRiotParams {
                    page: Some(1),
                    ..Default::default()
                })
                .await,
        ),
        (
            "minecraft",
            market
                .category_minecraft(MarketCategoryMinecraftParams {
                    page: Some(1),
                    ..Default::default()
                })
                .await,
        ),
    ] {
        println!("\n--- {name} ---");
        match result {
            Ok(data) => {
                if let Some(total) = data.get("totalItems") {
                    println!("total: {total}");
                }
            }
            Err(e) => eprintln!("  err: {e}"),
        }
    }

    println!("\n--- steam params ---");
    match market.category_params("steam".into()).await {
        Ok(p) => println!(
            "keys: {:?}",
            p.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- steam games ---");
    match market.category_games("steam".into()).await {
        Ok(g) => println!(
            "keys: {:?}",
            g.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- currency ---");
    match market.payments_currency().await {
        Ok(c) => println!("{}", serde_json::to_string_pretty(&c)?),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\ndone");
    Ok(())
}
