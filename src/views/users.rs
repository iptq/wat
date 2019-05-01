use diesel::prelude::*;
use diesel::result::Error::RollbackTransaction;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use uuid::{adapter::Hyphenated, Uuid};

use crate::captcha::{Captcha, CaptchaText};
use crate::context::Context;
use crate::db::DbConn;
use crate::models::{NewUser, User};

#[get("/login")]
pub fn login(ctx: Context, mut cookies: Cookies) -> Template {
    let mut ctx = ctx.into_inner();
    Template::render("login", &ctx)
}

#[derive(FromForm)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[post("/login", data = "<form>")]
pub fn post_login(conn: DbConn, form: Form<LoginForm>) -> Redirect {
    Redirect::to(uri!(super::stats::dashboard))
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

#[derive(FromForm)]
pub struct RegisterForm {
    email: String,
    display_name: Option<String>,
    password: String,
    captcha: String,
}

#[post("/register", data = "<form>")]
pub fn post_register(
    conn: DbConn,
    form: Form<RegisterForm>,
    captcha_text: CaptchaText,
    mut cookies: Cookies,
) -> Redirect {
    // TODO: validation
    let form = form.into_inner();

    let new_user = NewUser {
        email: form.email,
        display_name: form.display_name,
        password: form.password,
        api_key: generate_api_key(),
    };

    let user = conn
        .0
        .transaction(|| {
            use crate::schema::users::{self, dsl};
            match diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&conn.0)
            {
                Err(err) => return Err(RollbackTransaction),
                _ => println!("Inserted!"),
            };
            dsl::users
                .filter(dsl::email.eq(&new_user.email))
                .first(&conn.0)
        })
        .unwrap();

    login_user(&mut cookies, &user);
    Redirect::to(uri!(super::stats::dashboard))
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
