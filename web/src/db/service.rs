use anyhow::Result;
use easy_password::bcrypt::hash_password;
use sqlx::{SqlitePool, sqlite::SqliteQueryResult};

use super::models::User;

pub async fn user_by_username(
    username: &str,
    pool: &SqlitePool
) -> Result<User, sqlx::Error> {

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await;

    match user {
        Ok(result) => Ok(result),
        Err(e) => Err(e)
    }
}

pub async fn does_username_exist(username: &str, pool: &SqlitePool) -> bool {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_all(pool)
        .await;

    // This seems VERY silly.
    match users {
        Ok(result) => result.len() > 0,
        Err(_) => false
    }
}

pub async fn create_user(
    username: &str,
    password: &str,
    pool: &SqlitePool
) -> Result<SqliteQueryResult, sqlx::Error> {

    let hashed_pass = hash_password(password, b"dev-VERYsecure", 12).unwrap();
    sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
        .bind(username)
        .bind(hashed_pass)
        .execute(pool)
        .await
}
