mod api_key;
mod users;

use rocket::Route;

use self::api_key::ApiKey;

pub fn routes() -> Vec<Route> {
    routes![
        self::users::user_heartbeats,
        self::users::current_user_heartbeats,
        self::users::post_current_user_heartbeats,
    ]
}
