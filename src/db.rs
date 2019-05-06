use diesel::SqliteConnection;
use rocket_contrib::databases::diesel;

use crate::errors::Error;

embed_migrations!("migrations");

#[database("database")]
pub struct DbConn(SqliteConnection);

pub type PooledConn = SqliteConnection;

impl DbConn {
          pub fn migrate(&self) -> Result<(), Error> {
            embedded_migrations::run(&self.0).map_err(Error::from)
    }
}
