use chrono::Utc;
use diesel::prelude::*;
use rocket_contrib::json::Json;

use super::ApiKey;
use crate::db::{DbConn, PooledConn};
use crate::models::{Heartbeat, NewHeartbeat, User};

#[derive(Serialize)]
pub struct HeartbeatData {
    pub entity: String,
    pub entity_type: String,
    pub category: String,
    pub time: f64,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub language: Option<String>,
    pub dependencies: Option<String>,
    pub lines: i32,
    pub line_number: Option<i32>,
    pub cursor_pos: Option<i32>,
    pub is_write: bool,
}

#[derive(Serialize)]
pub struct HeartbeatResult {
    pub data: Vec<HeartbeatData>,
    pub start: i32,
    pub end: i32,
    pub timezone: String,
}

fn get_user_heartbeats(conn: &PooledConn, user: &User) -> Json<HeartbeatResult>{
    use crate::schema::heartbeats::dsl::{heartbeats, user_id};
    let hbs: Vec<Heartbeat> = heartbeats.filter(user_id.eq(user.id)).load(conn).unwrap();

    let result = HeartbeatResult {
        data: Vec::new(),
        start: 0,
        end: 0,
        timezone: "".to_string(),
    };
    Json(result)
}

#[get("/users/<user_id>/heartbeats")]
pub fn user_heartbeats(conn: DbConn, user_id: i32, _api_key: ApiKey) -> Json<HeartbeatResult> {
    let user = User::by_id(&conn.0, user_id).unwrap();
    get_user_heartbeats(&conn.0, &user)
}

#[get("/users/current/heartbeats")]
pub fn current_user_heartbeats(conn: DbConn, api_key: ApiKey) -> Json<HeartbeatResult> {
    let user = api_key.0;
    get_user_heartbeats(&conn.0, &user)
}

#[derive(Deserialize)]
pub struct PostHeartbeatData {
    pub entity: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub category: String,
    pub time: String,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub language: Option<String>,
    pub dependencies: Option<String>,
    pub lines: i32,
    pub lineno: Option<i32>,
    pub cursorpos: Option<i32>,
    pub is_write: Option<bool>,
}

#[derive(Serialize)]
pub struct PostHeartbeatResult {
    pub id: i32,
    pub entity: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub time: f64,
}

#[post("/users/current/heartbeats", format = "json", data = "<heartbeat>")]
pub fn post_current_user_heartbeats(
    conn: DbConn,
    api_key: ApiKey,
    heartbeat: Json<PostHeartbeatData>,
) -> Json<PostHeartbeatResult> {
    let user = api_key.0;
    let heartbeat = heartbeat.into_inner();
    let time = Utc::now().naive_utc();
    let new_heartbeat = NewHeartbeat {
        user_id: user.id,
        entity: heartbeat.entity,
        entity_type: heartbeat.entity_type,
        category: heartbeat.category,
        time: time,
        project: heartbeat.project,
        branch: heartbeat.branch,
        language: heartbeat.language,
        dependencies: heartbeat.dependencies,
        lines: heartbeat.lines,
        line_number: heartbeat.lineno,
        cursor_pos: heartbeat.cursorpos,
        is_write: heartbeat.is_write.unwrap_or_else(|| false),
    };

    use crate::schema::heartbeats;
    diesel::insert_into(heartbeats::table)
        .values(&new_heartbeat)
        .execute(&conn.0);

    let result = PostHeartbeatResult {
        id: 0,
        entity: new_heartbeat.entity,
        entity_type: new_heartbeat.entity_type,
        time: time.timestamp() as f64,
    };
    Json(result)
}
