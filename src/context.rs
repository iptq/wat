use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket_contrib::templates::tera::Context as Ctx;

use crate::Error;

pub struct Context(pub Ctx);

impl Context {
    pub fn into_inner(self) -> Ctx {
        self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut cookies = request.cookies();
        let user_id = cookies
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse::<i32>().ok());

        let mut ctx = Ctx::new();
        if let Some(user_id) = user_id {
            ctx.insert("user_id", &user_id);
        }
        Ok(Context(ctx)).or_forward(())
    }
}
