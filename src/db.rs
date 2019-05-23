use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error::{NotFound, RollbackTransaction};
use rocket_contrib::databases::diesel;

use crate::errors::Error;
use crate::models::{Heartbeat, NewUser, User};

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
        self.transaction(|| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&self.0)?;
            dsl::users
                .filter(dsl::email.eq(&new_user.email))
                .first(&self.0)
                .map_err(Error::from)
        })
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

    pub fn heartbeats_interval(
        &self,
        user_id: i32,
        start: NaiveDateTime,
        end: NaiveDateTime,
        project: Option<&str>,
    ) -> Result<Vec<Heartbeat>, Error> {
        use crate::schema::heartbeats::dsl;
        dsl::heartbeats
            .filter(
                dsl::user_id
                    .eq(user_id)
                    .and(dsl::time.ge(start))
                    .and(dsl::time.le(end)),
            )
            .load(&self.0)
            .map_err(Error::from)
    }
}
