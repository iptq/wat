mod auth;
mod heartbeats;
mod summary;

use rocket::Route;

use self::auth::Auth;

pub fn routes() -> Vec<Route> {
    routes![
        self::heartbeats::user_heartbeats,
        self::heartbeats::current_user_heartbeats,
        self::heartbeats::post_current_user_heartbeats,
        self::summary::user_summaries,
        self::summary::current_user_summaries,
    ]
}
