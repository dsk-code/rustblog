-- Add up migration script here
CREATE TABLE IF NOT EXISTS post_tags (
    post_id UUID NOT NULL REFERENCES posts (id) DEFERRABLE INITIALLY DEFERRED,
    tag_id INTEGER NOT NULL REFERENCES tags (id) DEFERRABLE INITIALLY DEFERRED,
    PRIMARY KEY (post_id, tag_id)
);