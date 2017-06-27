use std::ops::Deref;
use rocket::http::Status;
use rocket::{Request, State, Outcome};
use rocket::request::{self, FromRequest};
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;
use std::sync::Arc;

pub type Pool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

pub fn init_db_pool() -> Pool {
    let config = r2d2::Config::default();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Arc::new(r2d2::Pool::new(config, manager).expect("Failed to create pool."))
}

pub struct DB(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for DB {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DB, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(DB(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
