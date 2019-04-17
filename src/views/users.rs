use rocket::response::Redirect;
use rocket_contrib::templates::{tera::Context, Template};

#[get("/register")]
pub fn register() -> Template {
    let ctx = Context::new();
    Template::render("register", &ctx)
}

#[post("/register")]
pub fn post_register() -> Redirect {
    Redirect::to(uri!(register))
}
