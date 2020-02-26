use super::models::*;

use std::env;
use diesel::prelude::*;
use diesel::dsl::sum;
use diesel::pg::PgConnection;
use diesel::sql_query;
use diesel::sql_types::Integer;


pub fn get_connection() -> PgConnection {
  let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    conn
}

pub fn get_entries(uid: i32, project_name: &str) -> Vec<LogEntry> {
    use super::schema::log_entry::dsl::{log_entry, user_id, project_id};

    let conn = get_connection();
    let id = find_project_by_name(project_name).id;

    log_entry
        .filter(user_id.eq(uid))
        .filter(project_id.eq(id))
        .load::<LogEntry>(&conn)
        .expect("Error loading posts")
}

pub fn create_entry(user_id: i32, hours: f32, project_name: Option<&String>) -> LogEntry {
    use super::schema::log_entry;
    let conn = get_connection();

    let project_id = if let Some(project_name) = project_name {
        Option::from(find_project_by_name(project_name).id)
    } else {
        Option::None
    };

    let new_entry = NewLogEntry {
        hours: hours,
        user_id: user_id,
        project_id: project_id,
    };

    diesel::insert_into(log_entry::table)
        .values(&new_entry)
        .get_result(&conn)
        .expect("Error saving new entry")
}

pub fn create_project(name: &str, user_id: i32) -> Project {
    use super::schema::project;
    let conn = get_connection();

    let new_project = NewProject {
        name: name,
        user_id: user_id,
    };

    diesel::insert_into(project::table)
        .values(&new_project)
        .get_result(&conn)
        .expect("Error saving new project")
}

pub fn find_project_by_name(project_name: &str) -> Project {
    use super::schema::project::dsl::*;
    let conn = get_connection();

    project
        .filter(name.eq(project_name))
        .first::<Project>(&conn)
        .expect("Could not find project")
}

pub fn get_projects(uid: i32) -> Vec<Project> {
    use super::schema::project::dsl::*;
    let conn = get_connection();

    project
        .filter(user_id.eq(uid))
        .load::<Project>(&conn)
        .expect("Error loading projects")
}

pub fn get_project_hours(project_name: &str) -> Option<f32> {
    use super::schema::log_entry::dsl::*;
    let conn = get_connection();

    let p_id = find_project_by_name(project_name).id;

    log_entry
        .filter(project_id.eq(p_id))
        .select(sum(hours))
        .first(&conn)
        .expect("could not sum hours")
}

pub fn hours_by_project(uid: i32) -> Vec<ProjectHours>{
    let conn = get_connection();

    sql_query(include_str!("queries/project_hours.sql"))
        .bind::<Integer, _>(uid)
        .load::<ProjectHours>(&conn)
        .expect("Could not find hours")
}