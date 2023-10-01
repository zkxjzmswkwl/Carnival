use sqlx::{SqlitePool, sqlite::SqliteQueryResult};

use crate::db::models::{User, QueuedPlayer};

pub async fn is_queued(
    queue_id: i32,
    username: &str,
    pool: &SqlitePool
) -> bool {

    let query = sqlx::query_file!("sql/is_queued.sql", username).fetch_all(pool).await;
    query.map(|x| !x.is_empty()).unwrap_or_default()
}

pub async fn add_user_to_queue(
    queue_id: i32,
    user_id: i32,
    role: &str,
    pool: &SqlitePool
) -> Result<SqliteQueryResult, sqlx::Error> {

    sqlx::query_file!(
        "sql/insert_queued_player.sql"queue_id, queue_id, user_id, role
    ).execute(pool).await
}

pub async fn delete_user_from_queue(
    queue_id: i32,
    user_id: i32,
    pool: &SqlitePool
) {

    sqlx::query_file!(
        "sql/delete_queued_player.sql", queue_id, user_id
    )
    .execute(pool)
    .await
    .map_err(|err| eprintln!("{err}"));
}

#[derive(Default, Debug)]
pub struct ResolvedQueuePlayer {
    pub role: String,
    pub username: String
}

pub async fn players_in_queue(
    queue_id: i32,
    pool: &SqlitePool
) -> Option<Vec<ResolvedQueuePlayer>> {

    let result = sqlx::query_file_as!(
        ResolvedQueuePlayer, "sql/resolve_queue.sql", queue_id
    ).fetch_all(pool).await;

    if let Ok(queued_players) = result {
        return Some(queued_players);
    } else {
        return None;
    }
}

#[derive(Default, Debug)]
pub struct ResolvedQueue {
    pub id: i32,
    pub title: String,
    pub tanks: Vec<ResolvedQueuePlayer>,
    pub dps: Vec<ResolvedQueuePlayer>,
    pub supports: Vec<ResolvedQueuePlayer>
}

impl ResolvedQueue {
    pub async fn from_id(
        queue_id: i32,
        pool: &SqlitePool
    ) -> Self {

        let mut ret = Self::default();
        ret.id = queue_id;

        let queued_players = players_in_queue(queue_id, pool).await;
        if queued_players.is_none() {
            return ret;
        }
        for player in queued_players.unwrap() {
            match player.role.as_str() {
                "Tank"    => ret.tanks.push(player),
                "DPS"     => ret.dps.push(player),
                "Support" => ret.supports.push(player),
                _ => println!("This should never happen.")
            }
        }
        return ret;
    }
}
