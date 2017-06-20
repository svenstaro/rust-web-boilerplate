#![feature(plugin, const_fn)]
#![plugin(rocket_codegen)]
#![cfg_attr(test, plugin(stainless))]

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

#[cfg(test)]
extern crate parking_lot;

pub mod api;
pub mod validation;
pub mod models;
pub mod schema;
pub mod handlers;
pub mod responses;
pub mod helpers;

#[cfg(test)]
mod tests;

use rocket::fairing::AdHoc;
use chrono::Duration;

pub struct RuntimeConfig(Duration);

pub fn rocket_factory() -> (rocket::Rocket, helpers::db::Pool) {
    let db_pool = helpers::db::init_db_pool();
    let rocket = rocket::ignite()
        .manage(db_pool.clone())
        .attach(AdHoc::on_attach(|rocket| {
            let auth_timeout = rocket.config().get_int("auth_token_timeout_days").unwrap_or(7);
            let auth_token_duration = Duration::days(auth_timeout);
            Ok(rocket.manage(RuntimeConfig(auth_token_duration)))
        }))
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
