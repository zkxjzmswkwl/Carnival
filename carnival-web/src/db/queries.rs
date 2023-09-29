pub mod tables {
    pub const CREATE_USERS: &'static str = "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY UNIQUE NOT NULL,
        username VARCHAR(250) UNIQUE NOT NULL,
        password VARCHAR(250) NOT NULL,
        battletag VARCHAR(17) UNIQUE NOT NULL);";

    pub const CREATE_SESSION_TOKENS: &'static str = "CREATE TABLE IF NOT EXISTS session_tokens (
        for_user INT NOT NULL,
        remote_addr VARCHAR(64) NOT NULL,
        unique_hmac_key VARCHAR(64) UNIQUE NOT NULL,
        token VARCHAR(250) UNIQUE NOT NULL,
        is_valid BIT NOT NULL);";
}
