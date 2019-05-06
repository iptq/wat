use rocket_contrib::templates::Template;

use crate::models::User;
use crate::utils::Context;

#[get("/dashboard")]
pub fn dashboard(ctx: Context, user: User) -> Template {
    let ctx = ctx.into_inner();
    Template::render("dashboard", &ctx)
}
