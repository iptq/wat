#[derive(FromForm)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}
