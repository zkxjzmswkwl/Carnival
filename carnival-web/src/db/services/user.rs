use anyhow::Result;
use easy_password::bcrypt::hash_password;
use sqlx::{SqlitePool, sqlite::SqliteQueryResult, Sqlite};

use crate::{HMAC_KEY, db::models::User};


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

pub async fn by_id(id: i32, pool: &SqlitePool) -> Option<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await;

    if user.is_ok() {
        return Some(user.unwrap());
    }
    None
}

pub async fn user_id_by_username(
    username: &str,
    pool: &SqlitePool
) -> Option<i32> {
    // Should just be id, but not sure what size of integer it'd be, since SQLite `INTEGER` is
    // dynamic, apparently.
    // From SQLite docs:
    //      INTEGER – 
    //      any numeric value is stored as a signed integer value (It can hold both positive and negative integer values).
    //      The INTEGER values in SQLite are stored in either 1, 2, 3, 4, 6, or 8 bytes of storage depending on the value of the number.
    let query_result: Result<i32, sqlx::Error> = sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await;

    if let Ok(unwrapped_userid) = query_result {
        return Some(unwrapped_userid);
    }
    None
}

async fn does_exist<T>(query: &str, column_value: &str, pool: &SqlitePool) -> bool
where
    T: for<'r> sqlx::FromRow<'r, <Sqlite as sqlx::Database>::Row>,
    T: Send + Unpin, 

{
    let users = sqlx::query_as::<_, T>(query)
        .bind(column_value)
        .fetch_all(pool)
        .await;

    users.map(|u| !u.is_empty()).unwrap_or_default()
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
