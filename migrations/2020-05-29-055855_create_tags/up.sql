CREATE TABLE Tags (
    tag_name TEXT NOT NULL,
    post_id INTEGER NOT NULL
        REFERENCES Posts (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    PRIMARY KEY (tag_name, post_id)
);
