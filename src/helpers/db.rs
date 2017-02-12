use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection, Config};
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}

// DB Items
lazy_static! {
	pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

pub struct DB(pub PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
	pub fn conn(&self) -> &PgConnection {
		&*self.0
	}
}

