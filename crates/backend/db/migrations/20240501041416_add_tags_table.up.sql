-- Add up migration script here
CREATE TABLE IF NOT EXISTS tags (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE
);