use chrono::Utc;
use diesel::prelude::*;
use rocket_contrib::json::Json;

use super::Auth;
use crate::db::{DbConn, PooledConn};
use crate::errors::Error;
use crate::models::{Heartbeat, NewHeartbeat, User};

#[derive(Serialize)]
pub struct HeartbeatData {
    pub entity: String,
    pub entity_type: String,
    pub category: Option<String>,
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

fn get_user_heartbeats(conn: &DbConn, user: &User) -> Json<HeartbeatResult> {
    use crate::schema::heartbeats::dsl::{heartbeats, user_id};
    let hbs: Vec<Heartbeat> = heartbeats
        .filter(user_id.eq(user.id))
        .load(&conn.0)
        .unwrap();

    let result = HeartbeatResult {
        data: Vec::new(),
        start: 0,
        end: 0,
        timezone: "".to_string(),
    };
    Json(result)
}

#[get("/users/<user_id>/heartbeats")]
pub fn user_heartbeats(conn: DbConn, user_id: i32, _auth: Auth) -> Json<HeartbeatResult> {
    let user = User::by_id(&conn, user_id).unwrap();
    get_user_heartbeats(&conn, &user)
}

#[get("/users/current/heartbeats")]
pub fn current_user_heartbeats(conn: DbConn, auth: Auth) -> Json<HeartbeatResult> {
    let user = auth.0;
    get_user_heartbeats(&conn, &user)
}

#[derive(Debug, Deserialize)]
pub struct PostHeartbeatData {
    pub entity: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub category: Option<String>,
    pub time: f64,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub language: Option<String>,
    pub dependencies: Vec<String>,
    pub lines: i32,
    pub lineno: Option<i32>,
    pub cursorpos: Option<i32>,
    pub is_write: Option<bool>,
}

#[derive(Serialize)]
pub struct PostHeartbeatResult {
    pub id: i32,
    pub time: f64,
}

#[post("/users/current/heartbeats", format = "json", data = "<heartbeats>")]
pub fn post_current_user_heartbeats(
    conn: DbConn,
    auth: Auth,
    heartbeats: Json<Vec<PostHeartbeatData>>,
) -> Result<Json<PostHeartbeatResult>, Error> {
    let user = auth.0;
    let heartbeats = heartbeats.into_inner();
    let time = Utc::now().naive_utc();

    use crate::schema::heartbeats;
    for heartbeat in heartbeats {
        let new_heartbeat = NewHeartbeat {
            user_id: user.id,
            entity: heartbeat.entity,
            entity_type: heartbeat.entity_type,
            category: heartbeat.category,
            time: time,
            project: heartbeat.project,
            branch: heartbeat.branch,
            language: heartbeat.language,
            dependencies: Some(heartbeat.dependencies.join(",")),
            lines: heartbeat.lines,
            line_number: heartbeat.lineno,
            cursor_pos: heartbeat.cursorpos,
            is_write: heartbeat.is_write.unwrap_or_else(|| false),
        };
        diesel::insert_into(heartbeats::table)
            .values(&new_heartbeat)
            .execute(&conn.0)?;
    }

    let result = PostHeartbeatResult {
        id: 0,
        time: time.timestamp() as f64,
    };
    Ok(Json(result))
}
