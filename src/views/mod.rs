mod base;
mod users;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![self::base::index,self::users::register]
}
