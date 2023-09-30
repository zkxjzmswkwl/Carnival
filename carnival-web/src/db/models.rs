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

impl User {}
