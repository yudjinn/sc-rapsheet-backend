-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    password VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE
)