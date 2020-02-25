use super::schema::log_entry;
use chrono::{NaiveDateTime};

#[derive(Insertable)]
#[table_name="log_entry"]
pub struct NewLogEntry {
    pub hours: f32,
    pub user_id: i32,
}

#[derive(Queryable)]
pub struct LogEntry {
    pub id: i32,
    pub hours: f32,
    pub time: NaiveDateTime,
    pub user_id: i32,
}