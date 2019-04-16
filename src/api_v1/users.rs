use super::ApiKey;
use crate::models::User;

fn get_user_heartbeats(user: &User) {}

#[get("/users/<id>/heartbeats")]
pub fn user_heartbeats(id: i32, key: ApiKey) {}

#[get("/users/current/heartbeats")]
pub fn current_user_heartbeats(key: ApiKey) {}
