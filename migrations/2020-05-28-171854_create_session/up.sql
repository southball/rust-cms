CREATE TABLE Sessions (
    session_id TEXT PRIMARY KEY,
    username TEXT NOT NULL
        REFERENCES Users (username)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    expiry TIMESTAMP NOT NULL
);
