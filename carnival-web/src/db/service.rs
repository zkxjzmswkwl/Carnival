use anyhow::Result;
use easy_password::bcrypt::hash_password;
use sqlx::{SqlitePool, sqlite::SqliteQueryResult, Sqlite};

use crate::HMAC_KEY;

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

async fn does_exist<T>(query: &str, column_value: &str, pool: &SqlitePool) -> bool
where
    T: for<'r> sqlx::FromRow<'r, <Sqlite as sqlx::Database>::Row>
{
    let users = sqlx::query_as::<_, User>(query)
        .bind(column_value)
        .fetch_all(pool)
        .await;

    users.map(|u| u.len() > 0).unwrap_or_default()
}

pub async fn does_username_exist(username: &str, pool: &SqlitePool) -> bool {
    does_exist::<User>("SELECT * FROM users WHERE username = $1", username, pool).await
}

pub async fn does_battletag_exist(battletag: &str, pool: &SqlitePool) -> bool {
    does_exist::<User>("SELECT * FROM users WHERE battletag = $1", battletag, pool).await
}

pub async fn create_user(
    username: &str,
    password: &str,
    battletag: &str,
    pool: &SqlitePool
) -> Result<SqliteQueryResult, sqlx::Error> {

    let hashed_pass = hash_password(password, HMAC_KEY, 12).unwrap();
    sqlx::query("INSERT INTO users (username, password, battletag) VALUES ($1, $2, $3)")
        .bind(username)
        .bind(hashed_pass)
        .bind(battletag)
        .execute(pool)
        .await
}
