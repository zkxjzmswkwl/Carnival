use sqlx::SqlitePool;

use super::services::{overwatch_match::{get_team, get_match_by_id}, user};

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct SessionToken {
    pub for_user: i32,
    pub remote_addr: String,
    pub unique_hmac_key: String,
    pub token: String,
    pub is_valid: bool,
    pub invalidation_source: String
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    username: String,
    password: String,
    battletag: String
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct OverwatchMap {
    pub id: i32,
    pub name: String,
    pub mode: String
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

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Default)]
pub struct OverwatchMatch {
    pub id: i32,
    pub map_id: i32,
}

#[derive(Default, Debug)]
pub struct ResolvedOverwatchMatch {
    pub overwatch_match: OverwatchMatch,
    pub blue_team: Vec<User>,
    pub red_team: Vec<User>
}

impl ResolvedOverwatchMatch {
    pub async fn from_id(ow_match_id: i32, pool: &SqlitePool) -> Self {
        if let Ok(ow_match) = get_match_by_id(ow_match_id, pool).await {
            // Resolve user objects for all players (blue/red_team)
            let blue_result = get_team(ow_match_id, 1, pool).await;
            let red_result = get_team(ow_match_id, 2, pool).await;

            if blue_result.is_ok() && red_result.is_ok() {
                // Get user_id from each owmatchplayer obj
                let blue_user_ids = blue_result.unwrap().iter().map(|x| x.user_id).collect();
                let red_user_ids = red_result.unwrap().iter().map(|x| x.user_id).collect();
                // From those userids, get Vec<Users> for each team
                let blue_user_objects = user::from_vec_ids(&blue_user_ids, pool).await.map_err(|e| eprintln!("{e}"));
                let red_user_objects = user::from_vec_ids(&red_user_ids, pool).await;

                if blue_user_objects.is_ok() && red_user_objects.is_ok() {
                    return Self {
                        overwatch_match: ow_match,
                        blue_team: blue_user_objects.unwrap(),
                        red_team: red_user_objects.unwrap() 
                    };
                }
            }
        }
        return ResolvedOverwatchMatch::default();
    }
}

#[allow(dead_code)]
impl User {
    pub fn get_username(&self)  -> &str { &self.username }
    pub fn get_password(&self)  -> &str { &self.password }
    pub fn get_battletag(&self) -> &str { &self.password }
}
