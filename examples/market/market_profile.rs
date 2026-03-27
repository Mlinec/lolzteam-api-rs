// cargo run --example market_profile -- YOUR_TOKEN
//
// профиль, баланс, история, корзина, избранное и т.д.

use lolzteam::market::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: market_profile <TOKEN>");
    let client = LolzteamClient::new(&token);
    let market = client.expect("failed to build client").market();

    println!("--- profile ---");
    match market.profile_get(None).await {
        Ok(resp) => {
            println!("{} (id: {})", resp.user.username, resp.user.user_id);
        }
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- payments ---");
    match market
        .payments_history(MarketPaymentsHistoryParams::default())
        .await
    {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- currency ---");
    match market.payments_currency().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- balance exchange ---");
    match market.payments_balance_list().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- favorites ---");
    match market
        .list_favorites(MarketListFavoritesParams::default())
        .await
    {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- orders ---");
    match market.list_orders(MarketListOrdersParams::default()).await {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- my items ---");
    match market.list_user(MarketListUserParams::default()).await {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- viewed ---");
    match market.list_viewed(MarketListViewedParams::default()).await {
        Ok(resp) => println!("total: {}", resp.total_items),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- cart ---");
    match market.cart_get(MarketCartGetParams::default()).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- states ---");
    match market.list_states(None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- proxy ---");
    match market.proxy_get().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- payout services ---");
    match market.payments_payout_services().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- discounts ---");
    match market.custom_discounts_get().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\n--- auto-payments ---");
    match market.auto_payments_list().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  err: {e}"),
    }

    println!("\ndone");
    Ok(())
}
