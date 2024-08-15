-- Add migration script here
-- TEST USER
INSERT INTO users (username, password, battletag, email, role) VALUES
    ('test123', 'orisa', 'a#123', 'tester@blizzard.com', 'Tank');

-- TEST QUEUE
INSERT INTO queues (title, demogrphic) VALUES ('Dev', 'shittas');

-- DEFAULT BRACKET
insert into brackets (queue_id) values(1);

-- DEFAULT BRACKET KEY
insert into bracket_keys (bracket_id, key) values (1, 'DefaultKey'); 