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
extern crate frank_jwt;
extern crate chrono;
extern crate argon2rs;

mod api;
mod validation;
mod models;
mod schema;

use rocket_contrib::{JSON, Value};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


#[error(400)]
fn bad_request() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Bad request."
    }))
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Not found."
    }))
}

fn main() {
    rocket::ignite()
        .mount("/api/hello/", routes![api::hello::whoami])
        .mount("/api/auth/", routes![
               api::auth::login,
               api::auth::register,
        ])
        .catch(errors![not_found, bad_request])
        .launch();
}
