use crate::models::User;

#[get("/dashboard")]
pub fn dashboard(user: User) {}
