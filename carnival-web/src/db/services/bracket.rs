use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

pub async fn add_user(
    user_id: i32,
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
