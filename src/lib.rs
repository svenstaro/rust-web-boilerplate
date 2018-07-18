#![feature(plugin)]
#![plugin(rocket_codegen)]
#![recursion_limit="128"]

extern crate uuid;
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
extern crate chrono;
extern crate argon2rs;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate ring;
extern crate rand;

pub mod api;
pub mod config;
pub mod handlers;
pub mod helpers;
pub mod models;
pub mod responses;
pub mod schema;
pub mod validation;

use std::sync::Arc;

/// Constructs a new Rocket instance.
///
/// This function takes care of attaching all routes and handlers of the application.
pub fn rocket_factory(config_name: &str) -> Result<(rocket::Rocket, helpers::db::Pool), String> {
    let config = config::Config::from(config_name)?;
    let db_pool = helpers::db::init_db_pool(&config.database_url).map_err(|e| e.to_string())?;
    let rocket = rocket::ignite()
        .manage(Arc::clone(&db_pool))
        .manage(config)
        .mount("/hello/", routes![api::hello::whoami])
        .mount(
            "/auth/",
            routes![
               api::auth::login,
               api::auth::register,
        ],
        )
        .catch(catchers![
            handlers::bad_request_handler,
            handlers::unauthorized_handler,
            handlers::forbidden_handler,
            handlers::not_found_handler,
            handlers::internal_server_error_handler,
            handlers::service_unavailable_handler,
        ]);
    Ok((rocket, db_pool))
}
