use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
    pub password_conf: String,
    pub battletag: String
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String
}
