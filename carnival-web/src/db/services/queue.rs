use sqlx::{SqlitePool, sqlite::SqliteQueryResult};

use crate::db::{models::OverwatchMatch, services::overwatch_match::{ResolvedTeams, create_match}};

pub async fn is_queued(
    queue_id: i32,
    username: &str,
    pool: &SqlitePool
) -> bool {

    let query = sqlx::query_file!("sql/is_queued.sql", username, queue_id).fetch_all(pool).await;
    query.map(|x| !x.is_empty()).unwrap_or_default()
}

pub async fn add_user_to_queue(
    queue_id: i32,
    user_id: i32,
    role: &str,
    pool: &SqlitePool
) -> Result<SqliteQueryResult, sqlx::Error> {

    sqlx::query_file!(
        "sql/insert_queued_player.sql", queue_id, user_id, role
    ).execute(pool).await
}

pub async fn delete_user_from_queue(
    queue_id: &i32,
    user_id: &i32,
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
    pub username: String,
    // User id
    pub id: i32,
    pub role: String
}

pub async fn players_in_queue(
    queue_id: i32,
    pool: &SqlitePool
) -> Option<Vec<ResolvedQueuePlayer>> {

    // query_file_as_unchecked! is required here
    // due to a bug in sqlx that has not been fixed in over 2 years.
    // NOTE: This does not disable compile-time query validation,
    // only type checking. Fuck sqlx.
    let result = sqlx::query_file_as_unchecked!(
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

pub async fn pop_queue(
    queue_id: i32,
    pool: &SqlitePool
) -> Option<i32> {

    async fn constraints_satisfied(queue: &ResolvedQueue) -> bool {
        println!("{}, {}, {}", queue.tanks.len(), queue.dps.len(), queue.supports.len());
        queue.tanks.len()    >= 2 &&
        queue.dps.len()      >= 4 &&
        queue.supports.len() >= 4
    }

    // NOTE: This currently does not take into account any sort of MMR.
    // It assigns players to blue/red team in order that they queued (for their role).
    fn assign_teams(queue: &ResolvedQueue) -> (Vec<i32>, Vec<i32>) {
        // Blue team
        let mut blue: Vec<i32> = vec![];
        // 1 Tank
        blue.push(queue.tanks[0].id);
        // 2 DPS
        for dps_player in queue.dps[0..2].iter() {
            blue.push(dps_player.id);
        }
        // 2 Supports
        for support_player in queue.supports[0..2].iter() {
            blue.push(support_player.id);
        }

        // Red team
        let mut red: Vec<i32> = vec![];
        // 1 Tank
        red.push(queue.tanks[1].id);
        // 2 DPS
        for dps_player in queue.dps[2..4].iter() {
            red.push(dps_player.id);
        }
        // 2 Supports
        for support_player in queue.supports[2..4].iter() {
            red.push(support_player.id);
        }

        (blue, red)
    }

    let queue = ResolvedQueue::from_id(queue_id, pool).await;
    // Ensure that enough players to fill a match are queued.
    // LOL
    if constraints_satisfied(&queue).await {
        println!("Constraints satisfied, queue pop.");
        let teams = assign_teams(&queue);
        println!("assign_teams return\n{:#?}", teams);
        // TODO: Map selection
        let insertion_result = create_match(1, &teams.0, &teams.1, pool).await;
        match insertion_result {
            Ok(created_match) => return Some(created_match),
            Err(e) => {
                eprintln!("{e}");
                return None;
            }
        }
    }
    None
}
