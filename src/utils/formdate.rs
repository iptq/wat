use std::ops::Deref;

use chrono::{Date, TimeZone, Utc};
use rocket::http::RawStr;
use rocket::request::FromFormValue;

pub struct FormDate(Date<Utc>);

pub enum Error {
    ParseError(chrono::format::ParseError),
}

impl<'v> FromFormValue<'v> for FormDate {
    type Error = Error;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Error> {
        use chrono::NaiveDate;
        NaiveDate::parse_from_str(form_value.as_str(), "%Y-%m-%d")
            .map_err(Error::ParseError)
            .map(|date| Utc.from_local_date(&date).unwrap())
            .map(FormDate)
    }
}

impl FormDate {
    pub fn into_inner(self) -> Date<Utc> {
        self.0
    }
}

impl Deref for FormDate {
    type Target = Date<Utc>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
