#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "128"]

extern crate uuid;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate validator;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;
extern crate argon2rs;
extern crate chrono;
extern crate rand;
extern crate ring;

pub mod api;
pub mod config;
pub mod database;
pub mod handlers;
pub mod models;
pub mod responses;
pub mod schema;
pub mod validation;

/// Constructs a new Rocket instance.
///
/// This function takes care of attaching all routes and handlers of the application.
pub fn rocket_factory(config_name: &str) -> Result<rocket::Rocket, String> {
    let config = config::get_rocket_config(config_name).map_err(|x| format!("{}", x))?;
    let rocket = rocket::custom(config)
        .attach(database::DbConn::fairing())
        .mount("/hello/", routes![api::hello::whoami])
        .mount("/auth/", routes![api::auth::login, api::auth::register,])
        .register(catchers![
            handlers::bad_request_handler,
            handlers::unauthorized_handler,
            handlers::forbidden_handler,
            handlers::not_found_handler,
            handlers::internal_server_error_handler,
            handlers::service_unavailable_handler,
        ]);
    Ok(rocket)
}
