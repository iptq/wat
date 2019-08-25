use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use chrono::{DateTime, TimeZone, Utc};
use rocket::request::Form;
use rocket_contrib::json::Json;

use super::Auth;
use crate::db::DbConn;
use crate::errors::Error;
use crate::models::{Heartbeat, User};
use crate::utils::FormDate;

#[derive(FromForm)]
pub struct SummaryParams {
    start: FormDate,
    end: FormDate,
    project: Option<String>,
    branches: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct SummaryRange {
    date: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    text: String,
    timezone: String,
}

#[derive(Clone, Debug, Serialize)]
struct SummaryItem {
    projects: Vec<SummaryInnerItem>,
    languages: Vec<SummaryInnerItem>,
    editors: Vec<SummaryInnerItem>,
    operating_systems: Vec<SummaryInnerItem>,
    dependencies: Vec<SummaryInnerItem>,
    machines: Vec<SummaryInnerItem>,
    branches: Vec<SummaryInnerItem>,
    entities: Vec<SummaryInnerItem>,
    range: SummaryRange,
}

impl PartialEq for SummaryItem {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
    }
}

impl Eq for SummaryItem {}

impl Hash for SummaryItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.range.start.hash(state);
        self.range.end.hash(state);
    }
}

#[derive(Serialize)]
pub struct SummaryResult {
    data: Vec<SummaryItem>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
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
    out: &mut HashSet<SummaryItem>,
) {
    // get a guarantee that we have at least 2 heartbeats
    if heartbeats.len() < 2 {
        return;
    }

    let mut current_item_time = heartbeats[0].time;
    let projects = vec![];
    let languages = vec![];
    let editors = vec![];
    let operating_systems = vec![];
    let dependencies = vec![];
    let machines = vec![];
    let branches = vec![];
    let entities = vec![];
    let range = ();

    let mut prev_heartbeat = &heartbeats[0];
    for heartbeat in heartbeats.iter().skip(1) {
        let time_delta = heartbeat.time - prev_heartbeat.time;
        println!("time_delta: {:?}", time_delta);

        if time_delta.num_seconds() >= timeout as i64 {
            // time to create a new segment
            out.insert(SummaryItem {
                projects: projects.clone(),
                languages: languages.clone(),
                editors: editors.clone(),
                operating_systems: operating_systems.clone(),
                dependencies: dependencies.clone(),
                machines: machines.clone(),
                branches: branches.clone(),
                entities: entities.clone(),
                range: SummaryRange {
                    date: "".to_owned(),
                    text: "".to_owned(),
                    timezone: "".to_owned(),
                    start: current_item_time,
                    end: prev_heartbeat.time,
                },
            });

            current_item_time = heartbeat.time;
        }

        prev_heartbeat = heartbeat;
    }
    out.insert(SummaryItem {
        projects,
        languages,
        editors,
        operating_systems,
        dependencies,
        machines,
        branches,
        entities,
        range: SummaryRange {
            date: "".to_owned(),
            text: "".to_owned(),
            timezone: "".to_owned(),
            start: current_item_time,
            end: prev_heartbeat.time,
        },
    });
}

fn get_user_summaries(
    conn: &DbConn,
    user: &User,
    summary_params: SummaryParams,
) -> Result<Json<SummaryResult>, Error> {
    let start_dt = summary_params.start.and_hms(0, 0, 0);
    let end_dt = summary_params.end.and_hms(23, 59, 59);

    let heartbeats = conn.get_heartbeats_interval(user.id, start_dt, end_dt, None)?;
    let mut data = HashSet::new();

    calculate_stat(StatType::Project, &heartbeats, 15 * 60, &mut data);
    calculate_stat(StatType::Language, &heartbeats, 15 * 60, &mut data);
    calculate_stat(StatType::Editor, &heartbeats, 15 * 60, &mut data);
    calculate_stat(StatType::OperatingSystem, &heartbeats, 15 * 60, &mut data);

    let data = data.into_iter().collect();
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
    let activity = vec![
        Heartbeat {
            id: 1,
            user_id: 1,
            entity: "file1.rs".to_owned(),
            entity_type: "file".to_owned(),
            category: Some("coding".to_owned()),
            time: Utc.ymd(2019, 8, 22).and_hms(0, 0, 0),
            project: Some("wat".into()),
            branch: Some("master".into()),
            language: Some("Rust".into()),
            dependencies: None,
            lines: 128,
            line_number: None,
            cursor_pos: Some(1770),
            is_write: false,
        },
        Heartbeat {
            id: 2,
            user_id: 1,
            entity: "file1.rs".to_owned(),
            entity_type: "file".to_owned(),
            category: Some("coding".to_owned()),
            time: Utc.ymd(2019, 8, 22).and_hms(0, 1, 0),
            project: Some("wat".into()),
            branch: Some("master".into()),
            language: Some("Rust".into()),
            dependencies: None,
            lines: 128,
            line_number: None,
            cursor_pos: Some(1771),
            is_write: false,
        },
        Heartbeat {
            id: 3,
            user_id: 1,
            entity: "file1.rs".to_owned(),
            entity_type: "file".to_owned(),
            category: Some("coding".to_owned()),
            time: Utc.ymd(2019, 8, 22).and_hms(0, 2, 0),
            project: Some("wat".into()),
            branch: Some("master".into()),
            language: Some("Rust".into()),
            dependencies: None,
            lines: 130,
            line_number: None,
            cursor_pos: Some(1790),
            is_write: false,
        },
        Heartbeat {
            id: 4,
            user_id: 1,
            entity: "file1.rs".to_owned(),
            entity_type: "file".to_owned(),
            category: Some("coding".to_owned()),
            time: Utc.ymd(2019, 8, 22).and_hms(0, 15, 0),
            project: Some("wat".into()),
            branch: Some("master".into()),
            language: Some("Rust".into()),
            dependencies: None,
            lines: 131,
            line_number: None,
            cursor_pos: Some(1801),
            is_write: false,
        },
        Heartbeat {
            id: 5,
            user_id: 1,
            entity: "file1.rs".to_owned(),
            entity_type: "file".to_owned(),
            category: Some("coding".to_owned()),
            time: Utc.ymd(2019, 8, 22).and_hms(0, 16, 0),
            project: Some("wat".into()),
            branch: Some("master".into()),
            language: Some("Rust".into()),
            dependencies: None,
            lines: 132,
            line_number: None,
            cursor_pos: Some(1802),
            is_write: false,
        },
    ];

    let mut out = HashSet::new();
    calculate_stat(StatType::Project, &activity, 10 * 60, &mut out);
    println!("out(10): {:?}", out);

    out.clear();
    calculate_stat(StatType::Project, &activity, 15 * 60, &mut out);
    println!("out(15): {:?}", out);

    panic!();
}
