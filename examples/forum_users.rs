// cargo run --example forum_users -- YOUR_TOKEN

use lolzteam::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args().nth(1).expect("Usage: forum_users <TOKEN>");
    let client = LolzteamClient::new(&token);
    let forum = client.forum();

    // получаем юзера по id
    println!("--- user #1 ---");
    match forum.users_get(1, None).await {
        Ok(user) => {
            println!(
                "username: {}",
                user["user"]["username"].as_str().unwrap_or("?")
            );
            println!("registered: {}", user["user"]["user_register_date"]);
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    // поиск по нику
    println!("\n--- find AS7RIDENIED ---");
    match forum.users_find(Some("AS7RIDENIED".into()), None, None).await {
        Ok(found) => {
            if let Some(exact) = found.get("exact") {
                println!("exact: {}", exact["username"].as_str().unwrap_or("?"));
            }
            if let Some(users) = found.get("users").and_then(|u| u.as_array()) {
                println!("results: {}", users.len());
                for u in users.iter().take(3) {
                    println!(
                        "  {} (id: {})",
                        u["username"].as_str().unwrap_or("?"),
                        u["user_id"]
                    );
                }
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    // список юзеров
    println!("\n--- users list ---");
    match forum.users_list(Some(1), Some(5), None).await {
        Ok(list) => {
            if let Some(users) = list.get("users").and_then(|u| u.as_array()) {
                for u in users.iter().take(5) {
                    println!(
                        "  {} — {} msgs",
                        u["username"].as_str().unwrap_or("?"),
                        u["user_message_count"]
                    );
                }
            }
        }
        Err(e) => eprintln!("  skip (нужны права): {e}"),
    }

    // подписчики
    println!("\n--- followers user #1 ---");
    match forum.users_followers(1, None, None, None).await {
        Ok(data) => {
            if let Some(users) = data.get("users").and_then(|u| u.as_array()) {
                println!("count: {}", users.len());
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\n--- followings user #1 ---");
    match forum.users_followings(1, None, None, None).await {
        Ok(data) => {
            if let Some(users) = data.get("users").and_then(|u| u.as_array()) {
                println!("count: {}", users.len());
            }
        }
        Err(e) => eprintln!("  skip: {e}"),
    }

    // трофеи
    println!("\n--- trophies ---");
    match forum.users_trophies(1).await {
        Ok(data) => println!(
            "keys: {:?}",
            data.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    // таймлайн контента
    println!("\n--- user #1 timeline ---");
    match forum.users_contents(1, None, None).await {
        Ok(data) => println!(
            "keys: {:?}",
            data.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    // кастомные поля
    println!("\n--- custom fields ---");
    match forum.users_fields().await {
        Ok(data) => println!(
            "keys: {:?}",
            data.as_object().map(|m| m.keys().collect::<Vec<_>>())
        ),
        Err(e) => eprintln!("  skip: {e}"),
    }

    println!("\ndone");
    Ok(())
}
