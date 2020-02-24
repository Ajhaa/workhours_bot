-- Your SQL goes here
CREATE TABLE log_entry (
  id SERIAL PRIMARY KEY,
  hours REAL NOT NULL,
  time TIMESTAMP NOT NULL DEFAULT current_timestamp,
  user_id INTEGER NOT NULL
);