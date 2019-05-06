use rocket::response::Redirect;
use rocket_contrib::templates::{tera::Context, Template};

use crate::utils::Either::{self, A, B};
use crate::models::User;

#[get("/")]
pub fn index(user: Option<User>) -> Either<Redirect,Template> {
    let ctx = Context::new();
    if let Some(_) = user {
        return A(Redirect::to(uri!(super::stats::dashboard)));
    }
    B(Template::render("index", &ctx))
}
