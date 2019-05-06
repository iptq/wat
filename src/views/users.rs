use diesel::prelude::*;
use diesel::result::Error::RollbackTransaction;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use uuid::{adapter::Hyphenated, Uuid};

use crate::db::DbConn;
use crate::errors::{Error, UserError};
use crate::models::{NewUser, User};
use crate::utils::Context;
use crate::utils::{Captcha, CaptchaText};
use crate::views::forms::{LoginForm, RegisterForm};

#[get("/login")]
pub fn login(ctx: Context, mut cookies: Cookies) -> Template {
    let mut ctx = ctx.into_inner();
    Template::render("login", &ctx)
}

#[post("/login", data = "<form>")]
pub fn post_login(
    db: DbConn,
    form: Form<LoginForm>,
    mut cookies: Cookies,
) -> Result<Redirect, Error> {
    let user = match db.get_user_by_email(&form.email)? {
        Some(user) => user,
        None => return Err(Error::User(UserError::InvalidUsernameOrPassword)),
    };

    if !bcrypt::verify(&form.password, &user.password)? {
        return Err(Error::User(UserError::InvalidUsernameOrPassword));
    }

    login_user(&mut cookies, &user);
    Ok(Redirect::to(uri!(super::stats::dashboard)))
}

#[get("/register")]
pub fn register(ctx: Context, mut cookies: Cookies) -> Template {
    let mut ctx = ctx.into_inner();

    // generate a captcha
    let (captcha_text, captcha_img) = Captcha::new().as_tuple();

    // save captcha result in cookies
    cookies.add_private(Cookie::new("captcha_text", captcha_text));

    // add captcha image to context
    let captcha_img = "data:image/png;base64,".to_owned() + &base64::encode(&captcha_img);
    ctx.insert("captcha_img", &captcha_img);

    Template::render("register", &ctx)
}

#[post("/register", data = "<form>")]
pub fn post_register(
    db: DbConn,
    form: Form<RegisterForm>,
    captcha_text: CaptchaText,
    mut cookies: Cookies,
) -> Result<Redirect, Error> {
    // TODO: validation
    let form = form.into_inner();

    let password_hash = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST).map_err(Error::from)?;
    let new_user = NewUser {
        email: form.email,
        display_name: form.display_name,
        password: password_hash,
        api_key: generate_api_key(),
    };
    let user = db.insert_user(new_user)?;

    login_user(&mut cookies, &user);
    Ok(Redirect::to(uri!(super::stats::dashboard)))
}

#[get("/settings")]
pub fn settings(ctx: Context, user: User) -> Template {
    let mut ctx = ctx.into_inner();
    Template::render("settings", &ctx)
}

fn generate_api_key() -> String {
    let mut buf = [b'!'; Hyphenated::LENGTH];
    Uuid::new_v4()
        .to_hyphenated_ref()
        .encode_lower(&mut buf)
        .to_owned()
}

fn login_user(cookies: &mut Cookies, user: &User) {
    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
}
