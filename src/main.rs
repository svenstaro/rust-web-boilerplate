#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate uuid;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate validator;
#[macro_use] extern crate validator_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate jsonwebtoken;
extern crate chrono;
extern crate argon2rs;
extern crate rustc_serialize;

mod api;
mod validation;
mod models;
mod schema;
mod handlers;
mod responses;
mod helpers;

fn main() {
    rocket::ignite()
        .mount("/api/hello/", routes![api::hello::whoami])
        .mount("/api/auth/", routes![
               api::auth::login,
               api::auth::register,
        ])
        .catch(errors![handlers::not_found, handlers::bad_request])
        .launch();
}
