// cargo run --example forum_users -- YOUR_TOKEN

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: forum_users <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.forum();

    println!("--- user #1 ---");
    match forum.users_get(1, None).await {
        Ok(resp) => {
            println!("username: {}", resp.user.username);
            println!("registered: {}", resp.user.user_register_date);
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- find AS7RIDENIED ---");
    match forum
        .users_find(Some("AS7RIDENIED".into()), None, None)
        .await
    {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- users list ---");
    match forum.users_list(Some(1), Some(5), None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip (нужны права): {e}"),
    }

    println!("\n--- followers user #1 ---");
    match forum.users_followers(1, None, None, None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- followings user #1 ---");
    match forum.users_followings(1, None, None, None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- trophies ---");
    match forum.users_trophies(1).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- user #1 timeline ---");
    match forum.users_contents(1, None, None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- custom fields ---");
    match forum.users_fields().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
