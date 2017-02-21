use rocket::config::{Config, Environment, self};

pub fn get_secret<'a>() -> String {
    let default_secret = "secret".to_string();
    let conf = config::active().expect("No config found.");
    match (conf.get_str("secret"), Environment::active().unwrap()) {
        (Ok(s), _) => s.to_string(),
        (Err(_), Environment::Development) => default_secret,
        (Err(_), _) => panic!("A secret needs to be set unless you are in a development environment!")
    }
}
