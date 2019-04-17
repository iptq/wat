use diesel::SqliteConnection;
use rocket_contrib::databases::diesel;

#[database("database")]
pub struct DbConn(SqliteConnection);

pub type PooledConn = SqliteConnection;
