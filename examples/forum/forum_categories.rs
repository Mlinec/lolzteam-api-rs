// cargo run --example forum_categories -- YOUR_TOKEN

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: forum_categories <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.expect("failed to build client").forum();

    println!("--- categories ---");
    let cats = forum.categories_list(None, None, None).await?;
    println!("total: {}", cats.categories.len());
    for c in cats.categories.iter().take(5) {
        println!("  {:?}", c);
    }

    println!("\n--- category #1 ---");
    match forum.categories_get(1).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- forums ---");
    let forums = forum.forums_list(None, None, None).await?;
    println!("total: {}", forums.forums.len());
    for f in forums.forums.iter().take(5) {
        println!("  {:?}", f);
    }

    let forum_id = forums.forums.first().map(|f| f.forum_id).unwrap_or(1);

    println!("\n--- forum #{forum_id} ---");
    match forum.forums_get(forum_id).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- grouped forums ---");
    match forum.forums_grouped().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- navigation ---");
    match forum.navigation_list(None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- pages ---");
    match forum.pages_list(None, None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- followed forums ---");
    match forum.forums_followed(None).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- link forums ---");
    match forum.links_list().await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
