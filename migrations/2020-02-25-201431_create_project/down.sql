-- This file should undo anything in `up.sql`
ALTER TABLE log_entry DROP project_id;

DROP TABLE PROJECT;

