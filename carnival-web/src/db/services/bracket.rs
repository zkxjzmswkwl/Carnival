use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

use crate::db::models::{Bracket, BracketKey};

pub async fn add_user(
    user_id: i64,
    bracket_id: i32,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query_file!("sql/insert_user_into_bracket.sql", user_id, bracket_id)
        .execute(pool)
        .await
}

pub async fn add_bracket(
    queue_id: i32,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query_file!("sql/insert_bracket.sql", queue_id)
        .execute(pool)
        .await
}

pub async fn has_key(bracket_id: i32, pool: &SqlitePool) -> bool {
    match sqlx::query_file_as_unchecked!(BracketKey, "sql/bracket_has_key.sql", bracket_id)
        .fetch_one(pool)
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn does_key_exist(bracket_key: &str, pool: &SqlitePool) -> bool {
    match sqlx::query_file!("sql/does_bracket_key_exist.sql", bracket_key)
        .fetch_one(pool)
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

// This is way too similar to `does_key_exist` - Should just run a length check in `endpoints::register`.
pub async fn by_key(bracket_key: &str, pool: &SqlitePool) -> Option<Bracket> {
    match sqlx::query_file_as_unchecked!(Bracket, "sql/bracket_by_key.sql", bracket_key)
        .fetch_one(pool)
        .await {
            Ok(bracket) => {
                Some(bracket)
            }
            Err(e) => {
                eprintln!("{e}");
                None
            }
        }
}
