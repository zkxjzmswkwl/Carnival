pub mod tables {
    pub const CREATE_USERS: &str = "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY UNIQUE NOT NULL,
        role VARCHAR(30) NOT NULL,
        rating INTEGER DEFAULT 2500,
        wins INTEGER DEFAULT 0,
        losses INTEGER DEFAULT 0,
        username VARCHAR(250) UNIQUE NOT NULL,
        email VARCHAR(250) UNIQUE NOT NULL,
        password VARCHAR(250) NOT NULL,
        battletag VARCHAR(17) UNIQUE NOT NULL);";

    pub const CREATE_SESSION_TOKENS: &str = "CREATE TABLE IF NOT EXISTS session_tokens (
        for_user INT NOT NULL,
        remote_addr VARCHAR(100) NOT NULL,
        unique_hmac_key VARCHAR(100) UNIQUE NOT NULL,
        token VARCHAR(250) UNIQUE NOT NULL,
        is_valid BIT NOT NULL,
        invalidation_source VARCHAR(100));";

    pub const CREATE_OW_MAP: &str = "CREATE TABLE IF NOT EXISTS
        overwatch_maps (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            name VARCHAR(64) UNIQUE NOT NULL,
            mode VARCHAR(64) NOT NULL
        );";

    pub const CREATE_OW_MATCH_THRU: &str = "CREATE TABLE IF NOT EXISTS
        overwatch_match_players (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            user_id INTEGER NOT NULL,
            match_id INTEGER NOT NULL,
            team_id INTEGER NOT NULL
        );";

    pub const CREATE_OW_MATCH: &str = "CREATE TABLE IF NOT EXISTS
        overwatch_match (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            map_id INTEGER NOT NULL,
            winner INTEGER DEFAULT 0,
            status INTEGER DEFAULT 0
        );";

    pub const CREATE_QUEUE: &str = "CREATE TABLE IF NOT EXISTS
        queues (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            title VARCHAR(100) UNIQUE NOT NULL,
            demographic VARCHAR(100) UNIQUE NOT NULL
        );";

    pub const CREATE_QUEUED_PLAYERS: &str = "CREATE TABLE IF NOT EXISTS
        queued_players (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            queue_id INTEGER NOT NULL,
            user_id INTEGER UNIQUE NOT NULL,
            role VARCHAR(16) NOT NULL
        );";

    pub const CREATE_PASSWORD_RESET_TOKENS: & str = "CREATE TABLE IF NOT EXISTS
        password_reset_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,               -- Primary key implies not null & unique
            user_id INTEGER UNIQUE NOT NULL,                    -- foreign key to users table 1:1
            token VARCHAR(36) UNIQUE NOT NULL,                  -- can technically be char cause UUID length is fixed 
            created_at INTEGER DEFAULT (strftime('%s', 'now')), -- DATETIME exists but something something SQLx compatibility
            expires_at INTEGER NOT NULL,                        -- maybe we can make it compatible... but I cba
            FOREIGN KEY(user_id) REFERENCES users(id)           -- foreign key constraint
        );";

    pub const CREATE_BRACKETS: &str =
        "create table if not exists brackets(id integer primary key, queue_id not null);";
    pub const CREATE_BRACKETS_THRU: &str = "create table if not exists brackets_thru(id integer primary key, user_id integer not null, bracket_id integer not null);";
    pub const CREATE_BRACKET_KEYS: &str = "create table if not exists bracket_keys (id integer primary key not null, bracket_id integer not null, key varchar(250) not null);";
}
