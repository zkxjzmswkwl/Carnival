pub mod tables{
    pub const CREATE_USERS: &'static str = "CREATE TABLE IF NOT EXISTS users (
        username VARCHAR(250) UNIQUE NOT NULL,
        password VARCHAR(250) NOT NULL,
        battletag VARCHAR(17) UNIQUE NOT NULL);";
}
