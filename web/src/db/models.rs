use sqlx::Row;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    username: String,
    password: String
}

impl User {
    pub fn get_username(&self) -> &str { &self.username }
    pub fn get_password(&self) -> &str { &self.password }
}
