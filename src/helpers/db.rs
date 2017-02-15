use std::ops::Deref;
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;

pub fn init_db_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let config = r2d2::Config::default();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}

pub struct DB(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for DB {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
