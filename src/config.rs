use chrono::Duration;
use std::env;


#[derive(Debug)]
pub struct Config {
    pub auth_token_timeout_days: Duration,
    pub database_url: String,
    pub cors_allow_origin: String,
    pub cors_allow_methods: String,
    pub cors_allow_headers: String,
    pub environment_name: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            auth_token_timeout_days: Duration::days(30),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            cors_allow_origin: String::from("*"),
            cors_allow_methods: String::from("*"),
            cors_allow_headers: String::from("*"),
            environment_name: String::from("unconfigured"),
        }
    }
}

impl Config {
    pub fn from(config_name: &str) -> Result<Config, String> {
        // We'll start with a base config that sets some defaults and then apply the chosen app config.
        match config_name {
            "production" => Ok(Config::production_config()),
            "staging" => Ok(Config::staging_config()),
            "develop" => Ok(Config::develop_config()),
            "testing" => Ok(Config::testing_config()),
            "local" => Ok(Config::local_config()),
            _ => Err(format!("No valid config chosen: {}", config_name)),
        }
    }

    fn production_config() -> Config {
        Config {
            cors_allow_origin: String::from("https://example.com"),
            environment_name: String::from("production"),
            ..Default::default()
        }
    }

    fn staging_config() -> Config {
        Config {
            cors_allow_origin: String::from("https://staging.example.com"),
            environment_name: String::from("staging"),
            ..Default::default()
        }
    }

    fn develop_config() -> Config {
        Config{
            cors_allow_origin: String::from("https://develop.example.com"),
            environment_name: String::from("develop"),
            ..Default::default()
        }
    }

    fn testing_config() -> Config {
        Config{
            environment_name: String::from("testing"),
            ..Default::default()
        }
    }

    fn local_config() -> Config {
        Config{
            environment_name: String::from("local"),
            ..Default::default()
        }
    }
}
