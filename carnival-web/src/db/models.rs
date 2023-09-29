#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct SessionToken {
    for_user: i32,
    remote_addr: String,
    unique_hmac_key: String,
    token: String,
    is_valid: bool
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct User {
    id: i32,
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
