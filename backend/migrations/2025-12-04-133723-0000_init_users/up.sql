-- Your SQL goes here
CREATE TABLE users (
    id VARCHAR PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    created_at datetime NOT NULL DEFAULT (datetime('now'))
);