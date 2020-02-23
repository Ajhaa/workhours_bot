-- Your SQL goes here
CREATE TABLE log_entry (
  id SERIAL PRIMARY KEY,
  hours INTEGER,
  day DATE,
  user_id INTEGER
);