// cargo run --example basic -- YOUR_TOKEN

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: basic <TOKEN>");

    let client = LolzteamClient::new(&token);

    println!("--- forum: user #1 ---");
    match client.forum().users_get(1, None).await {
        Ok(data) => println!("{:#?}", data),
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\n--- forum: categories ---");
    match client.forum().categories_list(None, None, None).await {
        Ok(data) => println!("{:#?}", data),
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\n--- market: categories ---");
    match client.market().category_list(None).await {
        Ok(data) => println!("{:#?}", data),
        Err(e) => eprintln!("err: {e}"),
    }

    println!("\n--- market: payment history ---");
    match client.market().payments_history(Default::default()).await {
        Ok(data) => println!("{:#?}", data),
        Err(e) => eprintln!("err: {e}"),
    }

    Ok(())
}
