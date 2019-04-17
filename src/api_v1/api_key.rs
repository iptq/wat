use diesel::prelude::*;
use rocket::http::Status;
use rocket::outcome::{IntoOutcome, Outcome};
use rocket::request::{self, FromRequest, Request};

use crate::db::{DbConn, PooledConn};
use crate::models::User;

pub struct ApiKey(pub User);

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Invalid,
    Missing,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let auth = request.headers().get("authorization").collect::<Vec<_>>();
        match auth.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 => {
                let conn = request.guard::<DbConn>().unwrap();
                get_api_user(&conn.0, auth[0])
                    .map(|user| ApiKey(user))
                    .map_err(|_| ApiKeyError::Invalid)
                    .or_forward(())
            }
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

fn get_api_user(conn: &PooledConn, token: impl AsRef<str>) -> Result<User, ()> {
    use crate::schema::users::dsl::{api_key, users};
    let token = token.as_ref();
    users.filter(api_key.eq(token)).first(conn).map_err(|_| ())
}
