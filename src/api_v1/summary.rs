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

#[derive(Serialize)]
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

#[derive(Serialize)]
struct SummaryRange {
    date: String,
    start: u64,
    end: u64,
    text: String,
    timezone: String,
}

#[derive(Serialize, Default)]
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
    out: &mut HashMap<NaiveDateTime, SummaryItem>,
) {

    for window in heartbeats.as_slice().windows(2) {

    }
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

    calculate_stat(StatType::Project, &heartbeats, &mut data);
    calculate_stat(StatType::Language, &heartbeats, &mut data);
    calculate_stat(StatType::Editor, &heartbeats, &mut data);
    calculate_stat(StatType::OperatingSystem, &heartbeats, &mut data);

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
