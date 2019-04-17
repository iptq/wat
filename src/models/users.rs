use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Bool, HasSqlType};
use rocket::http::Cookie;
use rocket::outcome::{IntoOutcome, Outcome};
use rocket::request::{self, FromRequest, Request};

use crate::db::{DbConn, PooledConn};
use crate::Error;
use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub api_key: Option<String>,

    pub email_confirmed: bool,
    pub website: Option<String>,
    pub location: Option<String>,

    pub email_public: bool,
    pub logged_time_public: bool,
    pub languages_used_public: bool,

    pub last_heartbeat: Option<NaiveDateTime>,
    pub registered_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub display_name: Option<String>,
    pub password: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let conn = request.guard::<DbConn>().expect("no db connection?");
        let user_id = match request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse::<i32>().ok())
        {
            Some(user_id) => user_id,
            None => return Outcome::Forward(()),
        };
        User::by_id(&conn.0, user_id)
            .map_err(Error::Diesel)
            .or_forward(())
    }
}

impl User {
    pub fn by_id(conn: &PooledConn, user_id: i32) -> QueryResult<Self> {
        use crate::schema::users::dsl::{id, users};
        users.filter(id.eq(user_id)).first(conn)
    }
}
