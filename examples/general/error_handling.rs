// cargo run --example error_handling -- YOUR_TOKEN
//
// Детальная обработка всех типов ошибок.

use lolzteam::{Error, LolzteamClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::args()
        .nth(1)
        .expect("Usage: error_handling <TOKEN>");
    let client = LolzteamClient::new(&token).expect("failed to build client");

    // 1. Успешный запрос
    println!("--- successful request ---");
    match client.forum().users_get(1, None).await {
        Ok(resp) => println!("user: {} (id: {})", resp.user.username, resp.user.user_id),
        Err(e) => handle_error(&e),
    }

    // 2. 404 — несуществующий пользователь
    println!("\n--- not found (404) ---");
    match client.forum().users_get(999_999_999, None).await {
        Ok(resp) => println!("user: {}", resp.user.username),
        Err(e) => handle_error(&e),
    }

    // 3. 401 — невалидный токен
    println!("\n--- auth error (401) ---");
    let bad_client = LolzteamClient::new("invalid_token_12345").expect("failed to build client");
    match bad_client.forum().users_get(1, None).await {
        Ok(resp) => println!("user: {}", resp.user.username),
        Err(e) => handle_error(&e),
    }

    // 4. Использование status_code() для программной обработки
    println!("\n--- status_code() helper ---");
    match client.forum().users_get(999_999_999, None).await {
        Ok(_) => {}
        Err(ref e) => {
            if let Some(code) = e.status_code() {
                println!("HTTP status: {code}");
                match code {
                    401 => println!("  -> проверьте токен"),
                    403 => println!("  -> недостаточно прав"),
                    404 => println!("  -> ресурс не найден"),
                    429 => println!("  -> слишком много запросов"),
                    _ => println!("  -> неизвестная ошибка"),
                }
            }
        }
    }

    println!("\ndone");
    Ok(())
}

fn handle_error(err: &Error) {
    match err {
        Error::Auth { message } => {
            println!("[AUTH] Ошибка авторизации: {message}");
            println!("  Проверьте токен.");
        }
        Error::Forbidden { message } => {
            println!("[FORBIDDEN] Доступ запрещён: {message}");
            println!("  У токена нет прав на этот ресурс.");
        }
        Error::NotFound { message } => {
            println!("[NOT FOUND] Не найдено: {message}");
        }
        Error::RateLimited { attempts } => {
            println!("[RATE LIMIT] Лимит после {attempts} попыток.");
            println!("  Клиент уже сделал ретрай с backoff.");
        }
        Error::Api { status, body } => {
            println!("[API] HTTP {status}: {body}");
        }
        Error::Http(e) => {
            println!("[HTTP] Сетевая ошибка: {e}");
            if e.is_timeout() {
                println!("  Таймаут — попробуйте увеличить timeout.");
            } else if e.is_connect() {
                println!("  Не удалось подключиться — проверьте сеть/прокси.");
            }
        }
        Error::Json(e) => {
            println!("[JSON] Ошибка парсинга: {e}");
        }
    }
}
