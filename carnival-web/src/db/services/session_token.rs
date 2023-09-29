use std::net::SocketAddr;

use easy_password::bcrypt::{hash_password, verify_password};
use sqlx::{SqlitePool, sqlite::SqliteQueryResult};
use crate::db::models::SessionToken;
use rand::Rng;
use rand::distributions::Alphanumeric;

fn rand_str() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(25)
        .map(|c| c as char)
        .collect()
}

fn parse_remote_addr(connection: &SocketAddr) -> String {
    let mut remote_addr = connection.to_string();
    if remote_addr.contains(":") {
        remote_addr = remote_addr.split(":").next().unwrap().to_string();
    }

    remote_addr
}

pub async fn create(
    connection: &SocketAddr,
    user_id: i32,
    pool: &SqlitePool
) -> Option<String> {

    let remote_addr = parse_remote_addr(connection);
    // So apparently bcrypt has the concept of "cost". Which caps out at 31 or so.
    // Me personally, have no such concept on account of being extremely wealthy.
    // Doordash 20 times a week type money. You have no chance of getting it.
    // (This means we can't use a whole ass UUID as the hmac. Dang!)
    let unique_hmac = rand_str();
    let hashed_addr = hash_password(&remote_addr, unique_hmac.as_bytes(), 12).unwrap();
    // This looks shit. I think move **all** queries to queries.rs. That makes more sense anyway.
    let insert_result = sqlx::query("INSERT INTO session_tokens 
                (for_user, remote_addr, unique_hmac_key, token, is_valid)
                VALUES ($1, $2, $3, $4, $5);")
     .bind(user_id)
     .bind(&remote_addr)
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

pub async fn token_by_user_id(
    user_id: i32,
    pool: &SqlitePool
) -> Option<SessionToken> {

    let token = sqlx::query_as::<_, SessionToken>("SELECT * FROM session_tokens WHERE for_user = $1 AND is_valid = 1")
        .bind(user_id)
        .fetch_one(pool)
        .await;

    if token.is_ok() {
        return Some(token.unwrap());
    }
    None
}

pub async fn delete_by_user_id(
    user_id: i32,
    pool: &SqlitePool
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
    pool: &SqlitePool
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("UPDATE session_tokens SET is_valid = 0, invalidation_source = $1 WHERE for_user = $2;")
        .bind(invalidation_source)
        .bind(user_id)
        .execute(pool)
        .await
}

pub async fn validate(
    connection: &SocketAddr,
    user_id: i32,
    pool: &SqlitePool
) ->  Option<String> {
    if let Some(token) = token_by_user_id(user_id, pool).await {
        // Get remote addr without port
        let remote_addr = parse_remote_addr(connection);
        if let Ok(remotes_match) = verify_password(&remote_addr, &token.token, token.unique_hmac_key.as_bytes()) {
            if !remotes_match {
                // Someone's trying to use a session token bound to a different ip than their current.
                // Invalidate the session token, forcing the user to reauth with their password.
                let invalidate = set_invalid(user_id, &remote_addr, pool).await;
                match invalidate {
                    Ok(_) => println!("invalidation ok"),
                    Err(e) => eprintln!("{e}")
                }
                return None;
            }
            return Some(token.token);
        }
    }
    None
}
