// cargo run --example market_profile -- YOUR_TOKEN
//
// профиль, баланс, история, корзина, избранное и т.д.

use lolzteam::LolzteamClient;
use lolzteam::market::types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: market_profile <TOKEN>");
    let client = LolzteamClient::new(&token);
    let market = client.market();

    println!("--- profile ---");
    match market.profile_get(None).await {
        Ok(profile) => {
            if let Some(user) = profile.get("user") {
                println!("{} (id: {})", user["username"].as_str().unwrap_or("?"), user["user_id"]);
                println!("balance: {}", user["balance"]);
            }
        }
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- payments ---");
    match market.payments_history(MarketPaymentsHistoryParams::default()).await {
        Ok(payments) => {
            if let Some(arr) = payments.get("payments").and_then(|p| p.as_array()) {
                println!("{} payments", arr.len());
                for p in arr.iter().take(5) {
                    println!(
                        "  [{}] {} — {} ({})",
                        p["operation_id"],
                        p["label"].as_str().unwrap_or("?"),
                        p["incoming_sum"],
                        p["operation_type"].as_str().unwrap_or("?")
                    );
                }
            }
        }
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- currency ---");
    match market.payments_currency().await {
        Ok(curr) => println!("{}", serde_json::to_string_pretty(&curr)?),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- balance exchange ---");
    match market.payments_balance_list().await {
        Ok(ex) => println!(
            "keys: {:?}",
            ex.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- favorites ---");
    match market.list_favorites(MarketListFavoritesParams::default()).await {
        Ok(favs) => println!("total: {}", favs.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- orders ---");
    match market.list_orders(MarketListOrdersParams::default()).await {
        Ok(orders) => println!("total: {}", orders.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- my items ---");
    match market.list_user(MarketListUserParams::default()).await {
        Ok(items) => println!("total: {}", items.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- viewed ---");
    match market.list_viewed(MarketListViewedParams::default()).await {
        Ok(viewed) => println!("total: {}", viewed.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- cart ---");
    match market.cart_get(MarketCartGetParams::default()).await {
        Ok(cart) => println!(
            "keys: {:?}",
            cart.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- states ---");
    match market.list_states(None).await {
        Ok(s) => println!(
            "keys: {:?}",
            s.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- proxy ---");
    match market.proxy_get().await {
        Ok(p) => println!(
            "keys: {:?}",
            p.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- payout services ---");
    match market.payments_payout_services().await {
        Ok(ps) => println!(
            "keys: {:?}",
            ps.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- discounts ---");
    match market.custom_discounts_get().await {
        Ok(d) => println!(
            "keys: {:?}",
            d.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- auto-payments ---");
    match market.auto_payments_list().await {
        Ok(ap) => println!(
            "keys: {:?}",
            ap.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\ndone");
    Ok(())
}
