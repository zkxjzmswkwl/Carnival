CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY UNIQUE NOT NULL,
        role VARCHAR(30) NOT NULL,
        rating INTEGER DEFAULT 2500,
        wins INTEGER DEFAULT 0,
        losses INTEGER DEFAULT 0,
        username VARCHAR(250) UNIQUE NOT NULL,
        password VARCHAR(250) NOT NULL,
        email VARCHAR(250) UNIQUE NOT NULL,
        battletag VARCHAR(17) UNIQUE NOT NULL);
CREATE TABLE IF NOT EXISTS session_tokens (
        for_user INT NOT NULL,
        remote_addr VARCHAR(100) NOT NULL,
        unique_hmac_key VARCHAR(100) UNIQUE NOT NULL,
        token VARCHAR(250) UNIQUE NOT NULL,
        is_valid BIT NOT NULL,
        invalidation_source VARCHAR(100));
CREATE TABLE IF NOT EXISTS overwatch_match_players (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            user_id INTEGER NOT NULL,
            match_id INTEGER NOT NULL,
            team_id INTEGER NOT NULL
        );
CREATE TABLE IF NOT EXISTS overwatch_match (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            map_id INTEGER NOT NULL,
            winner INTEGER DEFAULT 0,
            status INTEGER DEFAULT 0
        );
CREATE TABLE IF NOT EXISTS overwatch_maps (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            name VARCHAR(64) UNIQUE NOT NULL,
            mode VARCHAR(64) NOT NULL
        );
CREATE TABLE IF NOT EXISTS queues (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            title VARCHAR(100) UNIQUE NOT NULL,
            demogrphic VARCHAR(100) UNIQUE NOT NULL
        );
CREATE TABLE IF NOT EXISTS queued_players (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            queue_id INTEGER NOT NULL,
            user_id INTEGER UNIQUE NOT NULL,
            role VARCHAR(16) NOT NULL
        );
CREATE TABLE IF NOT EXISTS
        password_reset_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,               -- Primary key implies not null & unique
            user_id INTEGER UNIQUE NOT NULL,                    -- foreign key to users table 1:1
            token VARCHAR(36) UNIQUE NOT NULL,                  -- can technically be char cause UUID length is fixed 
            created_at INTEGER DEFAULT (strftime('%s', 'now')), -- DATETIME exists but something something SQLx compatibility
            expires_at INTEGER NOT NULL,                        -- maybe we can make it compatible... but I cba
            FOREIGN KEY(user_id) REFERENCES users(id)           -- foreign key constraint
        );