use rocket_contrib::templates::{tera::Context, Template};

#[get("/register")]
pub fn register() -> Template {
    let ctx = Context::new();
    Template::render("index", &ctx)
}
