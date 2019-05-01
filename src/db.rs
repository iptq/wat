use diesel::SqliteConnection;
use rocket_contrib::databases::diesel;

embed_migrations!("migrations");

#[database("database")]
pub struct DbConn(SqliteConnection);

pub type PooledConn = SqliteConnection;
