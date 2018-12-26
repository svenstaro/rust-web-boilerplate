#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);
