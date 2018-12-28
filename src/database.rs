use rocket_contrib::database;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);
