use chrono::NaiveDateTime;
use rocket::request::Form;
use rocket_contrib::json::Json;

use super::Auth;
use crate::db::DbConn;
use crate::errors::Error;
use crate::models::User;
use crate::utils::NaiveDate;

#[derive(FromForm)]
pub struct SummaryParams {
    start: NaiveDate,
    end: NaiveDate,
    project: Option<String>,
    branches: Option<String>,
}

#[derive(Serialize)]
struct SummaryItem {}

#[derive(Serialize)]
pub struct SummaryResult {
    data: Vec<SummaryItem>,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

fn get_user_summaries(
    conn: &DbConn,
    user: &User,
    summary_params: SummaryParams,
) -> Result<Json<SummaryResult>, Error> {
    let mut data = Vec::new();
    let start_dt = summary_params.start.and_hms(0, 0, 0);
    let end_dt = summary_params.start.and_hms(23, 59, 59);

    let heartbeats = conn.heartbeats_interval(user.id, start_dt, end_dt, None);

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
