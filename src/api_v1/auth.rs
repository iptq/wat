use diesel::prelude::*;
use rocket::http::Status;
use rocket::outcome::{IntoOutcome, Outcome};
use rocket::request::{self, FromRequest, Request};

use crate::db::DbConn;
use crate::errors::{Error, UserError};
use crate::models::User;

/// Requires the user to auth either using the private cookie or API key, preferring the API key wherever applicable.
pub struct Auth(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // try using API key header
        let auth = request.headers().get("authorization").collect::<Vec<_>>();

        let result = match auth.len() {
            0 => None,
            1 => {
                let conn = request.guard::<DbConn>().unwrap();
                let value = auth[0];
                let result = base64::decode(value.split(" ").collect::<Vec<_>>()[1])
                    .map_err(Error::Base64)
                    .and_then(|v| String::from_utf8(v).map_err(Error::Utf8))
                    .and_then(|token| get_api_user(&conn, token).map_err(Error::Diesel))
                    .map(|user| Auth(user))
                    .map_err(|err| {
                        error!("ERROR: {:?}", err);
                        Error::User(UserError::BadApiKey)
                    })
                    .or_forward(());
                Some(result)
            }
            _ => {
                let err = Error::User(UserError::BadApiRequest);
                Some(Outcome::Failure((Status::BadRequest, err)))
            }
        };

        if let Some(result) = result {
            return result;
        }

        // try using request private cookies
        request.guard::<User>().map(Auth)
    }
}

fn get_api_user(conn: &DbConn, token: impl AsRef<str>) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::{api_key, users};
    let token = token.as_ref();
    info!("looking for token {}", token);
    users.filter(api_key.eq(token)).first(&conn.0)
}
