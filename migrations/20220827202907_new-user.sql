-- Add migration script here
CREATE TABLE users(
                      id uuid NOT NULL,
                      PRIMARY KEY (id),
                      name TEXT UNIQUE NOT NULL,
                      password TEXT NOT NULL,
                      added_at timestamptz NOT NULL
)