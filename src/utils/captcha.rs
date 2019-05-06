use captcha::{
    filters::{Dots, Noise, Wave},
    Captcha as Cap,
};
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

use crate::errors::Error;

pub struct Captcha(pub Cap);

impl Captcha {
    pub fn new() -> Self {
        let mut cap = Cap::new();
        cap.add_chars(6)
            .apply_filter(Noise::new(0.25))
            .apply_filter(Wave::new(2.0, 20.0).horizontal())
            .apply_filter(Wave::new(2.0, 20.0).vertical())
            .view(300, 100)
            .apply_filter(Dots::new(10));
        Captcha(cap)
    }

    pub fn as_tuple(&self) -> (String, Vec<u8>) {
        self.0.as_tuple().unwrap()
    }
}

#[derive(Debug)]
pub struct CaptchaText(pub Option<String>);

impl<'a, 'r> FromRequest<'a, 'r> for CaptchaText {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let text = request
            .cookies()
            .get_private("captcha_text")
            .map(|cookie| cookie.value().to_owned());
        Outcome::Success(CaptchaText(text))
    }
}
