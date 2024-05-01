-- Add up migration script here
CREATE TABLE IF NOT EXISTS post_categories (
    post_id UUID NOT NULL REFERENCES posts (id) DEFERRABLE INITIALLY DEFERRED,
    category_id INTEGER NOT NULL REFERENCES categories (id) DEFERRABLE INITIALLY DEFERRED,
    PRIMARY KEY (post_id, category_id)
);