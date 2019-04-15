use super::ApiKey;

fn get_user_heartbeats() {}

#[get("/users/<id>/heartbeats")]
pub fn user_heartbeats(id: i32, key: ApiKey) {}

#[get("/users/current/heartbeats")]
pub fn current_user_heartbeats(key: ApiKey) {}
