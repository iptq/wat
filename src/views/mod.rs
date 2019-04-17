mod base;
mod stats;
mod users;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        self::base::index,
        self::stats::dashboard,
        self::users::register,
    ]
}
