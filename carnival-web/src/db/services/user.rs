use anyhow::Result;
use easy_password::bcrypt::hash_password;
use headers::Cookie;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool};

use crate::{
    db::models::{SessionToken, User},
    HMAC_KEY,
};

use super::session_token::token_from_cookies;

pub async fn user_by_username(username: &str, pool: &SqlitePool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await;

    match user {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
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

pub async fn user_id_by_username(username: &str, pool: &SqlitePool) -> Option<i32> {
    // Should just be id, but not sure what size of integer it'd be, since SQLite `INTEGER` is
    // dynamic, apparently.
    // From SQLite docs:
    //      INTEGER –
    //      any numeric value is stored as a signed integer value (It can hold both positive and negative integer values).
    //      The INTEGER values in SQLite are stored in either 1, 2, 3, 4, 6, or 8 bytes of storage depending on the value of the number.
    let query_result: Result<i32, sqlx::Error> =
        sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
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

pub struct LeaderboardEntry {
    pub username: String,
    pub battletag: String,
    pub role: String,
    pub rating: i32,
    pub wins: i32,
    pub losses: i32,
}
pub async fn leaderboard_entries(pool: &SqlitePool) -> Result<Vec<LeaderboardEntry>, sqlx::Error> {
    sqlx::query_file_as_unchecked!(LeaderboardEntry, "sql/order_users_rating.sql")
        .fetch_all(pool)
        .await
}

pub async fn create_user(
    username: &str,
    password: &str,
    battletag: &str,
    role: &str,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let hashed_pass = hash_password(password, HMAC_KEY, 12).unwrap();
    sqlx::query_file!(
        "sql/insert_user.sql",
        username,
        hashed_pass,
        battletag,
        role
    )
    .execute(pool)
    .await
}

#[allow(dead_code)]
pub async fn from_vec_ids(
    user_id_list: &Vec<i32>,
    pool: &SqlitePool,
) -> Result<Vec<User>, sqlx::Error> {
    // (id1, id2, id3, etc)
    let mut sql_id_list: String = String::new();
    // This all seems very silly
    sql_id_list.push_str("(");
    for (idx, user_id) in user_id_list.iter().enumerate() {
        let id = format!("{},", user_id.to_string());
        // If it's not the last id, append `id,`
        if idx != user_id_list.len() - 1 {
            sql_id_list.push_str(&id);
        } else {
            // If it is, append a slice not including , `id`
            // This feels wrong so it probably is. 😎
            sql_id_list.push_str(&format!("{})", &id[0..id.len() - 1])[..]);
        }
    }

    sqlx::query_as::<_, User>(&format!("SELECT * FROM users WHERE id IN {}", sql_id_list)[..])
        .fetch_all(pool)
        .await
}

pub async fn by_token(session_token: &SessionToken, pool: &SqlitePool) -> Option<User> {
    // TODO: See if sqlx supports optional struct members (password, remote_addr).
    let user_result =
        sqlx::query_file_as_unchecked!(User, "sql/user_by_token.sql", session_token.token)
            .fetch_one(pool)
            .await;
    match user_result {
        Ok(user) => {
            Some(user)
        }
        Err(e) => {
            panic!("{e}");
        }
    }
    // if let Ok(result) = user_result {
    //     return Some(result);
    // }
}

pub async fn from_cookies(cookies: &Cookie, pool: &SqlitePool) -> Option<User> {
    println!("{:#?}", cookies);
    if let Some(session_token_option) = token_from_cookies(cookies) {
        if let Some(user) = by_token(&session_token_option, pool).await {
            return Some(user);
        }
    }
    return None;
}

pub async fn update_settings(user_id: i32, battletag: &str, role: &str, pool: &SqlitePool) {
    let query =
        sqlx::query_file!("sql/settings_user_update.sql", role, battletag, user_id).execute(pool).await;
    match query {
        Ok(o) => eprintln!("{:#?}", o),
        Err(e) => eprintln!("{e}")
    }
}
