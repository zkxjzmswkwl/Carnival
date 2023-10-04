use chrono::Utc;
use sqlx::{sqlite::SqliteQueryResult,  SqlitePool};

use crate::db::models::PasswordResetToken;

pub async fn store_token(
    user_id: i32,
    token: &str,
    expires_at: i64,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "INSERT INTO password_reset_tokens (user_id, token, expires_at) VALUES ($1, $2, $3);",
    )
    .bind(user_id)
    .bind(token)
    .bind(expires_at)
    .execute(pool)
    .await
}

pub async fn validate_token(
    token: &str,
    pool: &SqlitePool
) -> Result<Option<i32>, sqlx::Error> {
    let record: Option<PasswordResetToken> = sqlx::query_as("SELECT * FROM password_reset_tokens WHERE token = $1")
        .bind(token)
        .fetch_optional(pool).await?;

    if let Some(record) = record {
        if Utc::now().timestamp() < record.expires_at {
            return Ok(Some(record.user_id));
        }
    }
    Ok(None)
}

pub async fn delete_token(
    token: &str,
    pool: &SqlitePool
) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM password_reset_tokens WHERE token = $1")
        .bind(token)
        .execute(pool)
        .await
        // Returns number of rows affected... Should always be 1
        .map(|res| res.rows_affected())
}

pub async fn update_password(
    user_id: i32,
    new_password: &str,
    pool: &SqlitePool
) -> Result<(), sqlx::Error> {

    let rows_affected = sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
        .bind(new_password)
        .bind(user_id)
        .execute(pool)
        .await?
        .rows_affected();

    // Verify if the password was updated
    if rows_affected == 0 {
        // No rows were updated; this might indicate that the `user_id` is invalid
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}