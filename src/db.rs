use diesel::prelude::*;
use diesel::result::Error::{NotFound, RollbackTransaction};
use rocket_contrib::databases::diesel;

use crate::errors::Error;
use crate::models::{NewUser, User};

embed_migrations!("migrations");

#[database("database")]
pub struct DbConn(SqliteConnection);

pub type PooledConn = SqliteConnection;

impl DbConn {
    pub fn transaction<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce() -> Result<R, Error>,
    {
        let mut err = None;
        let result = self.0.transaction(|| match f() {
            Ok(v) => Ok(v),
            Err(e) => {
                err = Some(e);
                Err(RollbackTransaction)
            }
        });
        match result {
            Ok(v) => Ok(v),
            Err(_) => Err(err.unwrap()),
        }
    }

    pub fn migrate(&self) -> Result<(), Error> {
        embedded_migrations::run(&self.0).map_err(Error::from)
    }

    pub fn insert_user(&self, new_user: NewUser) -> Result<User, Error> {
        use crate::schema::users::{self, dsl};
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&self.0)?;
        dsl::users
            .filter(dsl::email.eq(&new_user.email))
            .first(&self.0)
            .map_err(Error::from)
    }

    pub fn get_user_by_email(&self, email: impl AsRef<str>) -> Result<Option<User>, Error> {
        use crate::schema::users::dsl;
        let email = email.as_ref();
        dsl::users
            .filter(dsl::email.eq(email))
            .first::<User>(&self.0)
            .map(|user| Some(user))
            .or_else(|err| match err {
                NotFound => Ok(None),
                _ => Err(err),
            })
            .map_err(Error::from)
    }
}
