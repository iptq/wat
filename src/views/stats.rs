use rocket_contrib::templates::Template;

use crate::context::Context;
use crate::models::User;

#[get("/dashboard")]
pub fn dashboard(ctx: Context, user: User) -> Template {
    let ctx = ctx.into_inner();
    Template::render("dashboard", &ctx)
}
