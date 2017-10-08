extern crate rust_web_boilerplate;

use std::env;

fn main() {
    let config_name = env::var("BOILERPLATEAPP_CONFIG").expect("BOILERPLATEAPP_CONFIG must be set");
    let (rocket, _) = rust_web_boilerplate::rocket_factory(&config_name).unwrap();
    rocket.launch();
}
