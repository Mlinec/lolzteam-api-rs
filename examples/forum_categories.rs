// cargo run --example forum_categories -- YOUR_TOKEN

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: forum_categories <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.forum();

    // категории
    println!("--- categories ---");
    let cats = forum.categories_list(None, None, None).await?;
    if let Some(arr) = cats.get("categories").and_then(|c| c.as_array()) {
        println!("total: {}", arr.len());
        for c in arr.iter().take(5) {
            println!(
                "  [{}] {}",
                c["category_id"],
                c["category_title"].as_str().unwrap_or("?")
            );
        }
    }

    let cat_id = cats
        .get("categories")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.first())
        .and_then(|c| c["category_id"].as_i64())
        .unwrap_or(1);

    println!("\n--- category #{cat_id} ---");
    match forum.categories_get(cat_id).await {
        Ok(cat) => {
            if let Some(c) = cat.get("category") {
                println!("title: {}", c["category_title"].as_str().unwrap_or("?"));
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    // форумы
    println!("\n--- forums ---");
    let forums = forum.forums_list(None, None, None).await?;
    if let Some(arr) = forums.get("forums").and_then(|f| f.as_array()) {
        println!("total: {}", arr.len());
        for f in arr.iter().take(5) {
            println!(
                "  [{}] {} ({} threads)",
                f["forum_id"],
                f["forum_title"].as_str().unwrap_or("?"),
                f["forum_thread_count"]
            );
        }
    }

    let forum_id = forums
        .get("forums")
        .and_then(|f| f.as_array())
        .and_then(|arr| arr.first())
        .and_then(|f| f["forum_id"].as_i64())
        .unwrap_or(1);

    println!("\n--- forum #{forum_id} ---");
    match forum.forums_get(forum_id).await {
        Ok(f) => {
            if let Some(data) = f.get("forum") {
                println!("{}", data["forum_title"].as_str().unwrap_or("?"));
                println!("{}", data["forum_description"].as_str().unwrap_or(""));
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- grouped forums ---");
    match forum.forums_grouped().await {
        Ok(g) => println!(
            "keys: {:?}",
            g.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    // навигация (может 500-тить)
    println!("\n--- navigation ---");
    match forum.navigation_list(None).await {
        Ok(nav) => {
            if let Some(arr) = nav.get("elements").and_then(|e| e.as_array()) {
                println!("{} items", arr.len());
                for n in arr.iter().take(5) {
                    println!("  {}", n["title"].as_str().unwrap_or("?"));
                }
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- pages ---");
    match forum.pages_list(None, None).await {
        Ok(pages) => println!(
            "keys: {:?}",
            pages.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- followed forums ---");
    match forum.forums_followed(None).await {
        Ok(followed) => {
            if let Some(arr) = followed.get("forums").and_then(|f| f.as_array()) {
                println!("count: {}", arr.len());
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- link forums ---");
    match forum.links_list().await {
        Ok(links) => println!(
            "keys: {:?}",
            links.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
