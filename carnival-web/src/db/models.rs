use sqlx::{SqlitePool, FromRow};

use super::services::{overwatch_match::{get_team, get_match_by_id}, user};

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct SessionToken {
    pub for_user: i32,
    pub remote_addr: String,
    pub unique_hmac_key: String,
    pub token: String,
    pub is_valid: bool,
    pub invalidation_source: String,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub battletag: String,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct OverwatchMap {
    pub id: i32,
    pub name: String,
    pub mode: String
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Default)]
pub struct OverwatchMatch {
    pub id: i32,
    pub map_id: i32,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct OverwatchMatchPlayer {
    pub id: i32,
    pub user_id: i32,
    pub match_id: i32,
    // Blue 1
    // Red  2
    pub team_id: u8
}

pub struct Queue {
    pub id: i32,
    pub title: String,
    pub demographic: String
}

#[derive(FromRow)]
pub struct QueuedPlayer {
    pub id: i32,
    pub queue_id: i32,
    pub user_id: i32,
    pub role: String
}
