use super::schema::log_entry;
use std::time::SystemTime;

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
    pub time: SystemTime,
    pub user_id: i32,
}