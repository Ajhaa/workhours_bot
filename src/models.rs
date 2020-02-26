use super::schema::*;
use diesel::sql_types::*;
use chrono::{NaiveDateTime};

#[derive(Insertable)]
#[table_name="log_entry"]
pub struct NewLogEntry {
    pub hours: f32,
    pub user_id: i32,
    pub project_id: Option<i32>,
}

#[derive(Queryable)]
pub struct LogEntry {
    pub id: i32,
    pub hours: f32,
    pub time: NaiveDateTime,
    pub user_id: i32,
    pub project_id: Option<i32>,
}

#[derive(QueryableByName)]
pub struct ProjectHours {
    #[sql_type="Text"]
    pub name: String,
    #[sql_type="Float"]
    pub hours: f32,
}

#[derive(Insertable)]
#[table_name="project"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub user_id: i32,
}

#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}
