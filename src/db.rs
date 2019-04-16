use rocket_contrib::databases::diesel;

#[database("database")]
pub struct Database(diesel::SqliteConnection);
