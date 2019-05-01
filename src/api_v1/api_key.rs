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

    Base64Decode(base64::DecodeError),
    StringDecode(std::string::FromUtf8Error),
    Database(diesel::result::Error),
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let auth = request.headers().get("authorization").collect::<Vec<_>>();

        match auth.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 => {
                let conn = request.guard::<DbConn>().unwrap();
                let value = auth[0];
                base64::decode(value.split(" ").collect::<Vec<_>>()[1])
                    .map_err(ApiKeyError::Base64Decode)
                    .and_then(|v| String::from_utf8(v).map_err(ApiKeyError::StringDecode))
                    .and_then(|token| get_api_user(&conn.0, token).map_err(ApiKeyError::Database))
                    .map(|user| ApiKey(user))
                    .map_err(|err| {
                        error!("ERROR: {:?}", err);
                        ApiKeyError::Invalid
                    })
                    .or_forward(())
            }
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

fn get_api_user(conn: &PooledConn, token: impl AsRef<str>) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::{api_key, users};
    let token = token.as_ref();
    info!("looking for token {}", token);
    users.filter(api_key.eq(token)).first(conn)
}
