// cargo run --example forum_search -- YOUR_TOKEN
//
// Поиск по форуму: темы, посты, теги, пользователи.

use lolzteam::forum::types::*;
use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: forum_search <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.forum();

    // Поиск тем
    println!("--- search threads: 'steam' ---");
    match forum
        .search_threads(ForumSearchThreadsParams {
            q: Some("steam".into()),
            limit: Some(5),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    // Поиск постов
    println!("\n--- search posts: 'гарант' ---");
    match forum
        .search_posts(ForumSearchPostsParams {
            q: Some("гарант".into()),
            limit: Some(5),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    // Поиск по тегам
    println!("\n--- search tagged: 'steam' ---");
    match forum
        .search_tagged(ForumSearchTaggedParams {
            tag: Some("steam".into()),
            limit: Some(5),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    // Поиск по всему
    println!("\n--- search all: 'аккаунт' ---");
    match forum
        .search_all(ForumSearchAllParams {
            q: Some("аккаунт".into()),
            limit: Some(5),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    // Поиск пользователей
    println!("\n--- search users: 'AS7' ---");
    match forum.search_users(Some("AS7".into())).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    // Поиск постов профиля
    println!("\n--- search profile posts ---");
    match forum
        .search_profile_posts(ForumSearchProfilePostsParams {
            q: Some("привет".into()),
            limit: Some(5),
            ..Default::default()
        })
        .await
    {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
