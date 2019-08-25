use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use crate::schema::heartbeats;

#[derive(Clone, Debug)]
pub struct Heartbeat {
    pub id: i32,
    pub user_id: i32,
    pub entity: String,
    pub entity_type: String,
    pub category: Option<String>,
    pub time: DateTime<Utc>,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub language: Option<String>,
    pub dependencies: Option<String>,
    pub lines: i32,
    pub line_number: Option<i32>,
    pub cursor_pos: Option<i32>,
    pub is_write: bool,
}

#[derive(Queryable)]
pub struct GetHeartbeat {
    pub id: i32,
    pub user_id: i32,
    pub entity: String,
    pub entity_type: String,
    pub category: Option<String>,
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

impl From<GetHeartbeat> for Heartbeat {
    fn from(heartbeat: GetHeartbeat) -> Self {
        Heartbeat {
            id: heartbeat.id,
            user_id: heartbeat.user_id,
            entity: heartbeat.entity,
            entity_type: heartbeat.entity_type,
            category: heartbeat.category,
            time: Utc.from_local_datetime(&heartbeat.time).unwrap(),
            project: heartbeat.project,
            branch: heartbeat.branch,
            language: heartbeat.language,
            dependencies: heartbeat.dependencies,
            lines: heartbeat.lines,
            line_number: heartbeat.line_number,
            cursor_pos: heartbeat.cursor_pos,
            is_write: heartbeat.is_write,
        }
    }
}

#[derive(Insertable)]
#[table_name = "heartbeats"]
pub struct NewHeartbeat {
    pub user_id: i32,
    pub entity: String,
    pub entity_type: String,
    pub category: Option<String>,
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
