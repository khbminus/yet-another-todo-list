-- Add migration script here
CREATE TABLE lists(
                      id uuid NOT NULL,
                      PRIMARY KEY (id),
                      name TEXT NOT NULL,
                      added_at timestamptz NOT NULL
)