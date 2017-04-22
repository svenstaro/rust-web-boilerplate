#![feature(plugin)]
#![plugin(stainless)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate rust_web_boilerplate;

mod test_api_auth;
