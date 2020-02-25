-- Your SQL goes here
CREATE TABLE project (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

ALTER TABLE log_entry
    ADD project_id INTEGER REFERENCES project(id);