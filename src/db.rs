use super::models::{LogEntry, NewLogEntry};

use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;


fn get_connection() -> PgConnection {
  let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    conn
}

pub fn get_entries() -> Vec<LogEntry> {
    use super::schema::log_entry::dsl::*;

    let conn = get_connection();

    log_entry
        .load::<LogEntry>(&conn)
        .expect("Error loading posts")
}

pub fn create_entry(user_id: i32, hours: f32) -> LogEntry {
    use super::schema::log_entry;
    let conn = get_connection();

    let new_entry = NewLogEntry {
        hours: hours,
        user_id: user_id,
    };

    diesel::insert_into(log_entry::table)
        .values(&new_entry)
        .get_result(&conn)
        .expect("Error saving new entry")
}