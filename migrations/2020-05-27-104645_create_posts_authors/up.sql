CREATE TABLE Users (
    username TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    last_update TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE Posts (
    id SERIAL PRIMARY KEY,
    draft BOOLEAN NOT NULL,
    publish_time TIMESTAMP NOT NULL,
    slug TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    author TEXT REFERENCES Users (username)
        ON DELETE CASCADE
        ON UPDATE RESTRICT
);

CREATE TABLE Config (
    config_name TEXT PRIMARY KEY,
    config_value TEXT NOT NULL
);
