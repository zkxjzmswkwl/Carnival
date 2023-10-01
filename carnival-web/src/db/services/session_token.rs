use crate::db::models::SessionToken;
use easy_password::bcrypt::{hash_password, verify_password};
use headers::Cookie;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};
use std::net::SocketAddr;

fn rand_str() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(25)
        .map(|c| c as char)
        .collect()
}

pub async fn create(connection: &SocketAddr, user_id: i32, pool: &SqlitePool) -> Option<String> {
    // So apparently bcrypt has the concept of "cost". Which caps out at 31 or so.
    // Me personally, have no such concept on account of being extremely wealthy.
    // Doordash 20 times a week type money. You have no chance of getting it.
    // (This means we can't use a whole ass UUID as the hmac. Dang!)

    let mut unique_hmac = String::from("fuey500");
    if !cfg!(debug_assertions) {
        unique_hmac = rand_str();
    }

    // TODO: This can panic >_<

    // For generated test account (see database_inator.sh)
    let mut remote_addr = String::from("127.0.0.1");
    if !cfg!(debug_assertions) {
        remote_addr = connection.ip().to_string();
    }

    let hashed_addr = hash_password(&remote_addr, unique_hmac.as_bytes(), 12).unwrap();

    // This looks shit. I think move **all** queries to queries.rs. That makes more sense anyway.
    let insert_result = sqlx::query(
        "INSERT INTO session_tokens 
                (for_user, remote_addr, unique_hmac_key, token, is_valid)
                VALUES ($1, $2, $3, $4, $5);",
    )
    .bind(user_id)
    .bind(connection.ip().to_string())
    .bind(&unique_hmac)
    .bind(&hashed_addr)
    .bind(true)
    .execute(pool)
    .await;

    match insert_result {
        Ok(_) => Some(hashed_addr),
        Err(e) => {
            eprintln!("{e}");
            None
        }
    }
}

pub async fn token_by_user_id(user_id: i32, pool: &SqlitePool) -> Option<SessionToken> {
    let token = sqlx::query_as::<_, SessionToken>(
        "SELECT * FROM session_tokens WHERE for_user = $1 AND is_valid = 1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await;

    if token.is_ok() {
        return Some(token.unwrap());
    }
    None
}

#[allow(dead_code)]
pub async fn delete_by_user_id(
    user_id: i32,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("DELETE from session_tokens WHERE for_user = $1")
        .bind(user_id)
        .execute(pool)
        .await
}

/// Flips byte for colum `is_valid`.
pub async fn set_invalid(
    user_id: i32,
    invalidation_source: &str,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "UPDATE session_tokens SET is_valid = 0, invalidation_source = $1 WHERE for_user = $2;",
    )
    .bind(invalidation_source)
    .bind(user_id)
    .execute(pool)
    .await
}

pub async fn validate(connection: &SocketAddr, user_id: i32, pool: &SqlitePool) -> Option<String> {
    if let Some(token) = token_by_user_id(user_id, pool).await {
        // Get remote addr without port
        if let Ok(remotes_match) = verify_password(
            &connection.ip().to_string(),
            &token.token,
            token.unique_hmac_key.as_bytes(),
        ) {
            if !remotes_match {
                // Someone's trying to use a session token bound to a different ip than their current.
                // Invalidate the session token, forcing the user to reauth with their password.
                let invalidate = set_invalid(user_id, &connection.ip().to_string(), pool).await;
                match invalidate {
                    Ok(_) => println!("invalidation ok"),
                    Err(e) => eprintln!("{e}"),
                }
                return None;
            }
            return Some(token.token);
        }
    }
    None
}

pub fn token_from_cookies(cookies: &Cookie) -> Option<String> {
    let session_option = cookies.get("session_id");
    if session_option.is_none() {
        return None;
    }
    let token = &session_option.unwrap()["Bearer ".len()..].to_string();
    Some(token.to_string())
}
