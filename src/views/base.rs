use rocket_contrib::templates::{tera::Context, Template};

#[get("/")]
pub fn index() -> Template {
    let ctx = Context::new();
    Template::render("index", &ctx)
}
