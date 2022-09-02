-- Add migration script here
CREATE TABLE tasks(
    id SERIAL PRIMARY KEY,
    list_id uuid,
    content TEXT NOT NULL,
    done boolean,
    CONSTRAINT fk_list_id
                  FOREIGN KEY(list_id)
                    REFERENCES lists(id)
)