#![feature(plugin, const_fn)]
#![plugin(stainless)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate diesel;
extern crate parking_lot;
extern crate uuid;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate rust_web_boilerplate;

mod factories;
mod test_api_auth;
mod test_api_hello;
