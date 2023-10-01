CREATE TABLE users (
        id INTEGER PRIMARY KEY UNIQUE NOT NULL,
        username VARCHAR(250) UNIQUE NOT NULL,
        password VARCHAR(250) NOT NULL,
        battletag VARCHAR(17) UNIQUE NOT NULL);
CREATE TABLE session_tokens (
        for_user INT NOT NULL,
        remote_addr VARCHAR(100) NOT NULL,
        unique_hmac_key VARCHAR(100) UNIQUE NOT NULL,
        token VARCHAR(250) UNIQUE NOT NULL,
        is_valid BIT NOT NULL,
        invalidation_source VARCHAR(100));
CREATE TABLE overwatch_match_players (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            user_id INTEGER NOT NULL,
            match_id INTEGER NOT NULL,
            team_id INTEGER NOT NULL
        );
CREATE TABLE overwatch_match (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            map_id INTEGER NOT NULL
        );
CREATE TABLE overwatch_maps (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            name VARCHAR(64) UNIQUE NOT NULL,
            mode VARCHAR(64) NOT NULL
        );
CREATE TABLE queues (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            title VARCHAR(100) UNIQUE NOT NULL,
            demogrphic VARCHAR(100) UNIQUE NOT NULL
        );
CREATE TABLE queued_players (
            id INTEGER PRIMARY KEY UNIQUE NOT NULL,
            queue_id INTEGER NOT NULL,
            user_id INTEGER UNIQUE NOT NULL,
            role VARCHAR(16) NOT NULL
        );
