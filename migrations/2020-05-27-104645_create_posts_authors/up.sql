CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    display_name TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE Posts (
    id SERIAL PRIMARY KEY,
    draft BOOLEAN NOT NULL,
    publish_time TIMESTAMP NOT NULL,
    slug TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    author INTEGER REFERENCES Users (id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT
);
