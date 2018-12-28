use rocket::config::{Config, ConfigError, Environment, Value};
use std::env;
use std::collections::HashMap;
use chrono::Duration;


#[derive(Debug)]
pub struct AppConfig {
    pub auth_token_timeout_days: Duration,
    pub cors_allow_origin: String,
    pub cors_allow_methods: String,
    pub cors_allow_headers: String,
    pub environment_name: String,
}

impl Default for AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            auth_token_timeout_days: Duration::days(30),
            cors_allow_origin: String::from("*"),
            cors_allow_methods: String::from("*"),
            cors_allow_headers: String::from("*"),
            environment_name: String::from("unconfigured"),
        }
    }
}


/// Return a tuple of an app-specific config and a Rocket config.
pub fn get_rocket_config(config_name: &str) -> Result<(AppConfig, Config), ConfigError> {
    fn production_config() -> Result<(AppConfig, Config), ConfigError> {
        let app_config = AppConfig {
            cors_allow_origin: String::from("https://example.com"),
            environment_name: String::from("production"),
            ..Default::default()
        };

        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        let rocket_config = Config::build(Environment::Production)
            .address("0.0.0.0")
            .port(8080)
            .extra("databases", databases)
            .finalize()?;

        Ok((app_config, rocket_config))
    }

    fn staging_config() -> Result<(AppConfig, Config), ConfigError> {
        let app_config = AppConfig {
            cors_allow_origin: String::from("https://staging.example.com"),
            environment_name: String::from("staging"),
            ..Default::default()
        };

        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        let rocket_config = Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(8080)
            .extra("databases", databases)
            .finalize()?;

        Ok((app_config, rocket_config))
    }

    fn develop_config() -> Result<(AppConfig, Config), ConfigError> {
        let app_config = AppConfig {
            cors_allow_origin: String::from("https://develop.example.com"),
            environment_name: String::from("develop"),
            ..Default::default()
        };

        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        let rocket_config = Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(8080)
            .extra("databases", databases)
            .finalize()?;

        Ok((app_config, rocket_config))
    }

    fn testing_config() -> Result<(AppConfig, Config), ConfigError> {
        let app_config = AppConfig {
            environment_name: String::from("testing"),
            ..Default::default()
        };

        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        let rocket_config = Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(5000)
            .extra("databases", databases)
            .finalize()?;

        Ok((app_config, rocket_config))
    }

    fn local_config() -> Result<(AppConfig, Config), ConfigError> {
        let app_config = AppConfig {
            environment_name: String::from("local"),
            ..Default::default()
        };

        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        let rocket_config = Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(5000)
            .extra("databases", databases)
            .finalize()?;

        Ok((app_config, rocket_config))
    }

    match config_name {
        "production" => production_config(),
        "staging" => staging_config(),
        "develop" => develop_config(),
        "testing" => testing_config(),
        "local" => local_config(),
        _ => Err(ConfigError::BadEnv(format!(
            "No valid config chosen: {}",
            config_name
        ))),
    }
}
