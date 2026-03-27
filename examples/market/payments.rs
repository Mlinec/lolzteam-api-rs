// cargo run --example market/payments -- YOUR_TOKEN

use lolzteam::market::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: payments <TOKEN>");
    let client = LolzteamClient::new(&token);
    let market = client.market();

    println!("--- баланс ---");
    match market.profile_get(None).await {
        Ok(resp) => {
            println!("баланс: {} руб", resp.user.balance);
            println!("hold: {} руб", resp.user.hold);
        }
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\n--- история платежей ---");
    match market
        .payments_history(MarketPaymentsHistoryParams {
            ..Default::default()
        })
        .await
    {
        Ok(resp) => {
            if let Some(payments) = resp.payments.as_array() {
                println!("{} операций", payments.len());
                for payment in payments.iter().take(10) {
                    if let (Some(id), Some(ptype), Some(amount)) = (
                        payment.get("payment_id").and_then(|v| v.as_i64()),
                        payment.get("type").and_then(|v| v.as_str()),
                        payment.get("amount").and_then(|v| v.as_f64()),
                    ) {
                        let comment = payment
                            .get("comment")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        println!("  [{}] {} - {} руб ({})", id, ptype, amount, comment);
                    }
                }
            }
        }
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\ndone");
    Ok(())
}
