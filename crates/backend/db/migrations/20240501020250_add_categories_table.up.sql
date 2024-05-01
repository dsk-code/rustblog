-- Add migration script here
CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT NOT NULL
);