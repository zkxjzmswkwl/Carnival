use sqlx::{SqlitePool, sqlite::SqliteQueryResult};

use crate::db::models::{User, QueuedPlayer};
use super::user;

pub async fn add_user_to_queue(
    queue_id: i32,
    user_id: i32,
    role: &str,
    pool: &SqlitePool
) -> Result<SqliteQueryResult, sqlx::Error> {

    sqlx::query!(
        "INSERT INTO queued_players (queue_id, user_id, role) VALUES (?, ?, ?);",
        queue_id, user_id, role
    ).execute(pool).await
}



// queue_id  id  role     username      title
// --------  --  -------  ------------  ----------
// 1         1   Tank     1realuserwow  Test queue
// 1         2   DPS      cartertest    Test queue
// 1         3   Support  carter123     Test queue
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

        let queued_players_opt = players_in_queue(queue_id, pool).await;
        if let Some(queued_players) = queued_players_opt {
            for player in queued_players {
                match player.role.as_str() {
                    "Tank"    => ret.tanks.push(player),
                    "DPS"     => ret.dps.push(player),
                    "Support" => ret.supports.push(player),
                    _ => println!("This should never happen.")
                }
            }
        }
        ret
    }
}
