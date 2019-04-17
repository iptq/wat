use chrono::NaiveDateTime;

use crate::schema::heartbeats;

#[derive(Queryable)]
pub struct Heartbeat {
    pub id: i32,
    pub user_id: i32,
    pub entity: String,
    pub entity_type: String,
    pub category: String,
    pub time: NaiveDateTime,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub language: Option<String>,
    pub dependencies: Option<String>,
    pub lines: i32,
    pub line_number: Option<i32>,
    pub cursor_pos: Option<i32>,
    pub is_write: bool,
}

#[derive(Insertable)]
#[table_name = "heartbeats"]
pub struct NewHeartbeat {
    pub user_id: i32,
    pub entity: String,
    pub entity_type: String,
    pub category: String,
    pub time: NaiveDateTime,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub language: Option<String>,
    pub dependencies: Option<String>,
    pub lines: i32,
    pub line_number: Option<i32>,
    pub cursor_pos: Option<i32>,
    pub is_write: bool,
}
