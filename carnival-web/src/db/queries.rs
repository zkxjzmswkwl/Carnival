pub mod tables {
    pub const CREATE_USERS: &str = "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY UNIQUE NOT NULL,
        role VARCHAR(30) NOT NULL,
        username VARCHAR(250) UNIQUE NOT NULL,
        password VARCHAR(250) NOT NULL,
        battletag VARCHAR(17) UNIQUE NOT NULL);";

    pub const CREATE_SESSION_TOKENS: &str = "CREATE TABLE IF NOT EXISTS session_tokens (
        for_user INT NOT NULL,
        remote_addr VARCHAR(100) NOT NULL,
        unique_hmac_key VARCHAR(100) UNIQUE NOT NULL,
        token VARCHAR(250) UNIQUE NOT NULL,
        is_valid BIT NOT NULL,
        invalidation_source VARCHAR(100));";

    pub const CREATE_OW_MAP: &'static str = "CREATE TABLE IF NOT EXISTS
        overwatch_maps (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            name VARCHAR(64) UNIQUE NOT NULL,
            mode VARCHAR(64) NOT NULL
        );";

    pub const CREATE_OW_MATCH_THRU: &'static str = "CREATE TABLE IF NOT EXISTS
        overwatch_match_players (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            user_id INTEGER NOT NULL,
            match_id INTEGER NOT NULL,
            team_id INTEGER NOT NULL
        );";

    pub const CREATE_OW_MATCH: &'static str = "CREATE TABLE IF NOT EXISTS
        overwatch_match (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            map_id INTEGER NOT NULL
        );"; 
    
    pub const CREATE_QUEUE: &'static str = "CREATE TABLE IF NOT EXISTS
        queues (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            title VARCHAR(100) UNIQUE NOT NULL,
            demographic VARCHAR(100) UNIQUE NOT NULL
        );";
    
    pub const CREATE_QUEUED_PLAYERS: &'static str = "CREATE TABLE IF NOT EXISTS
        queued_players (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            queue_id INTEGER NOT NULL,
            user_id INTEGER UNIQUE NOT NULL,
            role VARCHAR(16) NOT NULL
        );";
}
