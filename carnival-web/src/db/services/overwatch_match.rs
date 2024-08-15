use serde::Serialize;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

use crate::db::{
    models::{OverwatchMatch, OverwatchMatchPlayer},
    services::queue::delete_user_from_queue,
};

#[allow(dead_code)]
pub async fn create_map(
    map_name: &str,
    map_mode: &str,
    pool: &SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "INSERT INTO overwatch_maps (name, mode)
        VALUES ($1, $2);",
    )
    .bind(map_name)
    .bind(map_mode)
    .execute(pool)
    .await
}

pub async fn set_match_status(match_id: i32, match_status: u32,  pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query_file!("sql/set_match_status.sql", match_status, match_id).execute(pool).await
}

pub async fn get_pending_match(pool: &SqlitePool) -> Option<ResolvedOverwatchMatch> {
    let match_row = sqlx::query_file_as_unchecked!(OverwatchMatch, "sql/get_pending_match.sql")
        .fetch_one(pool)
        .await;

    if match_row.is_err() {
        return None;
    }

    let resolved_match = ResolvedOverwatchMatch::from_id(match_row.unwrap().id, pool).await;
    println!("{:#?}", resolved_match);
    Some(resolved_match)
}

#[allow(dead_code)]
pub async fn create_match(
    map_id: i32,
    blue_team: &[i32],
    red_team: &[i32],
    pool: &SqlitePool,
) -> Result<i32, sqlx::Error> {
    // Because we should **never** be inserting rows into the
    // `overwatch_match_players` table for **any other reason**.
    // If you do, you die ingame. 🔫🔫
    async fn insert_match_player(
        user_id: &i32,
        match_id: &i32,
        team: u8,
        pool: &SqlitePool,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO overwatch_match_players (user_id, match_id, team_id)
            VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(match_id)
        .bind(team)
        .execute(pool)
        .await
    }

    // Insert match before inserting thru rows for players
    let _match_result = sqlx::query(
        "
        INSERT INTO overwatch_match (map_id)
        VALUES ($1);",
    )
    .bind(map_id)
    .execute(pool)
    .await;

    let latest_match_result =
        sqlx::query!("SELECT id FROM (select (1) as id) overwatch_match ORDER BY id DESC LIMIT 1")
            .fetch_one(pool)
            .await;

    match latest_match_result {
        Ok(new_match) => {
            for player_id in blue_team.iter() {
                match insert_match_player(player_id, &new_match.id, 1, pool).await {
                    // TODO: Only support 1 queue at the moment.
                    Ok(_) => delete_user_from_queue(&1, player_id, pool).await,
                    Err(e) => eprintln!("{e}"),
                }
            }
            for player_id in red_team.iter() {
                match insert_match_player(player_id, &new_match.id, 2, pool).await {
                    // TODO: Only support 1 queue at the moment.
                    Ok(_) => delete_user_from_queue(&1, player_id, pool).await,
                    Err(e) => eprintln!("{e}"),
                }
            }
            Ok(new_match.id)
        }
        Err(e) => Err(e),
    }
}

pub async fn get_match_by_id(id: i32, pool: &SqlitePool) -> Result<OverwatchMatch, sqlx::Error> {
    sqlx::query_as::<_, OverwatchMatch>("SELECT * FROM overwatch_match WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

#[allow(dead_code)]
pub async fn get_match_players(
    match_id: i32,
    pool: &SqlitePool,
) -> Result<Vec<OverwatchMatchPlayer>, sqlx::Error> {
    sqlx::query_as::<_, OverwatchMatchPlayer>(
        "SELECT * FROM overwatch_match_players WHERE match_id = $1",
    )
    .bind(match_id)
    .fetch_all(pool)
    .await
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct ResolvedTeams {
    blue: Vec<String>,
    red: Vec<String>,
}
impl ResolvedTeams {
    // These shouldn't be vectors of strings
    // since they're being reallocated with a call to new.
    #[allow(dead_code)]
    pub fn new(blue: Vec<String>, red: Vec<String>) -> Self {
        Self { blue, red }
    }

    /// Intended for retrieving data to display.
    pub async fn for_match(ow_match_id: i32, pool: &SqlitePool) -> Self {
        let mut ret = ResolvedTeams::default();
        let blue: Vec<String> =
            sqlx::query_file_scalar_unchecked!("sql/resolve_match.sql", ow_match_id, 1)
                .fetch_all(pool)
                .await
                .unwrap_or_else(|err| {
                    eprintln!("{err}");
                    // :(
                    vec![":(".to_string()]
                });

        let red: Vec<String> =
            sqlx::query_file_scalar_unchecked!("sql/resolve_match.sql", ow_match_id, 2)
                .fetch_all(pool)
                .await
                .unwrap_or_else(|err| {
                    eprintln!("{err}");
                    // :(
                    vec![":(".to_string()]
                });

        ret.blue = blue;
        ret.red = red;
        ret
    }
}

#[allow(dead_code)]
pub async fn get_team(
    match_id: i32,
    team_id: u8,
    pool: &SqlitePool,
) -> Result<Vec<OverwatchMatchPlayer>, sqlx::Error> {
    sqlx::query_file_as_unchecked!(
        OverwatchMatchPlayer,
        "sql/players_by_team_match.sql",
        match_id,
        team_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct ResolvedOverwatchMatch {
    pub overwatch_match: OverwatchMatch,
    pub resolved_teams: ResolvedTeams,
}

impl ResolvedOverwatchMatch {
    #[allow(dead_code)]
    pub async fn from_id(ow_match_id: i32, pool: &SqlitePool) -> Self {
        let overwatch_match = get_match_by_id(ow_match_id, pool)
            .await
            .unwrap_or_else(|err| {
                eprintln!("{err}");
                OverwatchMatch::default()
            });
        let resolved_teams = ResolvedTeams::for_match(ow_match_id, pool).await;
        Self {
            overwatch_match,
            resolved_teams,
        }
    }
}
