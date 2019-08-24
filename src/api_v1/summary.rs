use std::collections::HashMap;

use chrono::NaiveDateTime;
use rocket::request::Form;
use rocket_contrib::json::Json;

use super::Auth;
use crate::db::DbConn;
use crate::errors::Error;
use crate::models::{Heartbeat, User};
use crate::utils::NaiveDate;

const THRESHOLD_SECONDS: u32 = 15 * 60; // 15 minutes

#[derive(FromForm)]
pub struct SummaryParams {
    start: NaiveDate,
    end: NaiveDate,
    project: Option<String>,
    branches: Option<String>,
}

#[derive(Debug, Serialize)]
struct SummaryInnerItem {
    name: String,
    total_seconds: f64,
    percent: f64,
    digital: String,
    text: String,
    hours: u32,
    minutes: u8,
    seconds: Option<u8>,
}

#[derive(Debug, Serialize)]
struct SummaryRange {
    date: String,
    start: u64,
    end: u64,
    text: String,
    timezone: String,
}

#[derive(Debug, Serialize, Default)]
struct SummaryItem {
    projects: Vec<SummaryInnerItem>,
    languages: Vec<SummaryInnerItem>,
    editors: Vec<SummaryInnerItem>,
    operating_systems: Vec<SummaryInnerItem>,
    dependencies: Vec<SummaryInnerItem>,
    machines: Vec<SummaryInnerItem>,
    branches: Vec<SummaryInnerItem>,
    entities: Vec<SummaryInnerItem>,
    range: Option<SummaryRange>,
}

#[derive(Serialize)]
pub struct SummaryResult {
    data: Vec<SummaryItem>,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

enum StatType {
    Project,
    Language,
    Editor,
    OperatingSystem,
}

fn calculate_stat(
    by: StatType,
    heartbeats: &Vec<Heartbeat>,
    timeout: u32,
    out: &mut HashMap<NaiveDateTime, SummaryItem>,
) {
    for window in heartbeats.as_slice().windows(2) {}
}

fn get_user_summaries(
    conn: &DbConn,
    user: &User,
    summary_params: SummaryParams,
) -> Result<Json<SummaryResult>, Error> {
    let start_dt = summary_params.start.and_hms(0, 0, 0);
    let end_dt = summary_params.end.and_hms(23, 59, 59);

    let heartbeats = conn.heartbeats_interval(user.id, start_dt, end_dt, None)?;
    let mut data = HashMap::new();

    calculate_stat(StatType::Project, &heartbeats, THRESHOLD_SECONDS, &mut data);
    calculate_stat(
        StatType::Language,
        &heartbeats,
        THRESHOLD_SECONDS,
        &mut data,
    );
    calculate_stat(StatType::Editor, &heartbeats, THRESHOLD_SECONDS, &mut data);
    calculate_stat(
        StatType::OperatingSystem,
        &heartbeats,
        THRESHOLD_SECONDS,
        &mut data,
    );

    let data = data.into_iter().map(|(_, item)| item).collect();
    let result = SummaryResult {
        data,
        start: start_dt,
        end: end_dt,
    };
    Ok(Json(result))
}

#[get("/users/<user_id>/summaries?<params..>")]
pub fn user_summaries(
    conn: DbConn,
    user_id: i32,
    params: Form<SummaryParams>,
    _auth: Auth,
) -> Result<Json<SummaryResult>, Error> {
    let params = params.into_inner();
    let user = User::by_id(&conn, user_id).unwrap();
    get_user_summaries(&conn, &user, params)
}

#[get("/users/current/summaries?<params..>")]
pub fn current_user_summaries(
    conn: DbConn,
    params: Form<SummaryParams>,
    auth: Auth,
) -> Result<Json<SummaryResult>, Error> {
    let user = auth.0;
    let params = params.into_inner();
    get_user_summaries(&conn, &user, params)
}

#[test]
fn test_calculate_stat() {
    use chrono::NaiveDate;

    let activity = vec![Heartbeat {
        id: 1,
        user_id: 1,
        entity: "file1.rs".to_owned(),
        entity_type: "file".to_owned(),
        category: Some("coding".to_owned()),
        time: NaiveDate::from_ymd(2019, 8, 22).and_hms(0, 0, 0),
        project: Some("wat".into()),
        branch: Some("master".into()),
        language: Some("Rust".into()),
        dependencies: None,
        lines: 128,
        line_number: None,
        cursor_pos: Some(1770),
        is_write: false,
    }];

    let mut out = HashMap::new();
    calculate_stat(StatType::Project, &activity, THRESHOLD_SECONDS, &mut out);
    panic!("out: {:?}", out);
}
