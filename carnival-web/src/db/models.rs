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
impl User {
    pub fn get_username(&self)  -> &str { &self.username }
    pub fn get_password(&self)  -> &str { &self.password }
    pub fn get_battletag(&self) -> &str { &self.password }
}
