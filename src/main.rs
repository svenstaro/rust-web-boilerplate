extern crate rust_web_boilerplate;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let config_name = env::var("CONFIG").expect("CONFIG must be set");
    let (rocket, _) = rust_web_boilerplate::rocket_factory(&config_name).unwrap();
    rocket.launch();
}
