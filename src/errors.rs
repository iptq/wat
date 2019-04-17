#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    User(UserError),
}

#[derive(Debug)]
pub enum UserError {}
