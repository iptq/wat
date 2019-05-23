use std::ops::Deref;

use rocket::http::RawStr;
use rocket::request::FromFormValue;

pub struct NaiveDate(chrono::NaiveDate);

impl<'v> FromFormValue<'v> for NaiveDate {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
        chrono::NaiveDate::parse_from_str(form_value.as_str(), "%Y-%m-%d")
            .map(NaiveDate)
            .map_err(|_| form_value)
    }
}

impl Deref for NaiveDate {
    type Target = chrono::NaiveDate;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
