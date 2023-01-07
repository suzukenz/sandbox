-- Add migration script here
CREATE TABLE labels (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);