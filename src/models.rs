#[derive(Queryable)]
pub struct LogEntry {
    pub id: i32,
    pub hours: i32,
    pub day: i32,
    pub published: bool,
}