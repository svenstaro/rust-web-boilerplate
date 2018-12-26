extern crate rust_web_boilerplate;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() -> Result<(), String> {
    dotenv().ok();

    let config_name = env::var("CONFIG_ENV").expect("CONFIG must be set");
    let rocket = rust_web_boilerplate::rocket_factory(&config_name)?;
    rocket.launch();
    Ok(())
}
