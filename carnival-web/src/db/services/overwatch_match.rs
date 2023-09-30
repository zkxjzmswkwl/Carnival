use sqlx::{SqlitePool, sqlite::SqliteQueryResult};

use crate::db::models::{OverwatchMatch, OverwatchMatchPlayer, User};

use super::user;

pub async fn create_map(
    map_name: &str,
    map_mode: &str,
    pool: &SqlitePool
) -> Result<SqliteQueryResult, sqlx::Error> {

    sqlx::query("INSERT INTO overwatch_maps (name, mode)
        VALUES ($1, $2);")
        .bind(map_name)
        .bind(map_mode)
        .execute(pool)
        .await
}

pub async fn create_match(
    map_id: i32,
    blue_team: &Vec<i32>,
    red_team: &Vec<i32>,
    pool: &SqlitePool
) -> Result<SqliteQueryResult, sqlx::Error> {

    // Because we should **never** be inserting rows into the
    // `overwatch_match_players` table for **any other reason**.
    // If you do, you die ingame. 🔫🔫
    async fn insert_match_player(
        user_id: &i32,
        match_id: &i32,
        team: u8,
        pool: &SqlitePool
    ) -> Result<SqliteQueryResult, sqlx::Error> {

        sqlx::query("
            INSERT INTO overwatch_match_players (user_id, match_id, team_id)
            VALUES ($1, $2, $3)")
            .bind(user_id)
            .bind(match_id)
            .bind(team)
            .execute(pool)
            .await
    }

    // Insert match before inserting thru rows for players
    let match_result = sqlx::query("
        INSERT INTO overwatch_match (map_id)
        VALUES ($1);")
        .bind(map_id)
        .execute(pool)
        .await;

    let latest_match_result = sqlx::query!(
        "SELECT id FROM (select (1) as id) overwatch_match ORDER BY id DESC LIMIT 1"
    ).fetch_one(pool).await;

    if let Ok(match_record) = latest_match_result {
        for player_id in blue_team.iter() {
            insert_match_player(player_id, &match_record.id, 1, pool).await.map_err(|err| eprintln!("{err}"));
        }
        for player_id in red_team.iter() {
            insert_match_player(player_id, &match_record.id, 2, pool).await.map_err(|err| eprintln!("{err}"));
        }
    }
    match_result
}

pub async fn get_match_by_id(
    id: i32,
    pool: &SqlitePool
) -> Result<OverwatchMatch, sqlx::Error> {
    sqlx::query_as::<_, OverwatchMatch>( 
        "SELECT * FROM overwatch_match WHERE id = $1"
    ).bind(id).fetch_one(pool).await
}

pub async fn get_match_players(
    match_id: i32,
    pool: &SqlitePool
) -> Result<Vec<OverwatchMatchPlayer>, sqlx::Error> {

    sqlx::query_as::<_, OverwatchMatchPlayer>("SELECT * FROM overwatch_match_players WHERE match_id = $1")
        .bind(match_id)
        .fetch_all(pool)
        .await
}

#[derive(Default, Debug)]
pub struct ResolvedTeams {
    blue: Vec<String>,
    red: Vec<String>
}
impl ResolvedTeams {
    pub async fn for_match(
        ow_match_id: i32,
        pool: &SqlitePool
    ) -> Self {

        let mut ret = ResolvedTeams::default();
        let blue: Vec<String> = sqlx::query_file_scalar_unchecked!(
            "sql/resolve_match.sql",
            ow_match_id,
            1
        )
        .fetch_all(pool)
        .await
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            // :(
            vec![":(".to_string()]
        });

        let red: Vec<String> = sqlx::query_file_scalar_unchecked!(
            "sql/resolve_match.sql",
            ow_match_id,
            1
        )
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

pub async fn get_team(
    match_id: i32,
    team_id: u8,
    pool: &SqlitePool
) -> Result<Vec<OverwatchMatchPlayer>, sqlx::Error> {

   sqlx::query_as::<_, OverwatchMatchPlayer>(
        "SELECT * FROM
        overwatch_match_players WHERE
        match_id = $1 AND
        team_id = $2"
    )
    .bind(match_id)
    .bind(team_id)
    .fetch_all(pool)
    .await
}

#[derive(Default, Debug)]
pub struct ResolvedOverwatchMatch {
    pub overwatch_match: OverwatchMatch,
    pub resolved_teams: ResolvedTeams
}

impl ResolvedOverwatchMatch {
    pub async fn from_id(ow_match_id: i32, pool: &SqlitePool) -> Self {
        let overwatch_match = get_match_by_id(ow_match_id, pool).await.unwrap_or_else(|err| {
            eprintln!("{err}");
            return OverwatchMatch::default();
        });
        let resolved_teams = ResolvedTeams::for_match(ow_match_id, pool).await;
        Self { overwatch_match, resolved_teams }
    }
}