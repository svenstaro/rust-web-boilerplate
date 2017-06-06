#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate uuid;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate validator;
#[macro_use] extern crate validator_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate chrono;
extern crate argon2rs;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate ring;

pub mod api;
pub mod validation;
pub mod models;
pub mod schema;
pub mod handlers;
pub mod responses;
pub mod helpers;

pub fn rocket_factory() -> (rocket::Rocket, helpers::db::Pool) {
    let mut db_pool = helpers::db::init_db_pool();
    let mut rocket = rocket::ignite()
        .manage(db_pool.clone())
        .mount("/hello/", routes![api::hello::whoami])
        .mount("/auth/", routes![
               api::auth::login,
               api::auth::register,
        ])
        .catch(errors![handlers::bad_request_handler, handlers::unauthorized_handler,
                       handlers::forbidden_handler, handlers::not_found_handler,
                       handlers::internal_server_error_handler,
                       handlers::service_unavailable_handler]);
    (rocket, db_pool)
}
