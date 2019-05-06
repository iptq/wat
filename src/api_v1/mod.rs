mod api_key;
mod heartbeats;
mod stats;

use rocket::Route;

use self::api_key::ApiKey;

pub fn routes() -> Vec<Route> {
    routes![
        self::heartbeats::user_heartbeats,
        self::heartbeats::current_user_heartbeats,
        self::heartbeats::post_current_user_heartbeats,
    ]
}
