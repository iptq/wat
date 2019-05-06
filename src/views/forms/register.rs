#[derive(FromForm)]
pub struct RegisterForm {
    pub email: String,
    pub display_name: Option<String>,
    pub password: String,
    pub captcha: String,
}
