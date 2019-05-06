use rocket::outcome::{IntoOutcome, Outcome};
use rocket::request::{self, FromRequest, Request};
use rocket_contrib::templates::tera::Context as Ctx;

use crate::errors::Error;
use crate::models::User;

pub struct Context(pub Ctx);

impl Context {
    pub fn into_inner(self) -> Ctx {
        self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut ctx = Ctx::new();
        info!("looking for a user");
        if let Outcome::Success(user) = request.guard::<User>() {
            ctx.insert("user", &user);
        }
        Ok(Context(ctx)).or_forward(())
    }
}
