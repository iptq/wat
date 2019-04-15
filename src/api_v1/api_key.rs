use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

pub struct ApiKey(String);

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
            1 if check_api_key(request, auth[0]) => Outcome::Success(ApiKey(auth[0].to_owned())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

fn check_api_key(request: &Request, token: impl AsRef<str>) -> bool {
    true
}
