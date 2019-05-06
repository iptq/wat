mod base;
mod forms;
mod stats;
mod users;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        self::base::index,
        self::stats::dashboard,
        self::users::login,
        self::users::post_login,
        self::users::register,
        self::users::post_register,
        self::users::settings,
    ]
}
