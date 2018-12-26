use rocket::config::{Config, ConfigError, Environment, Value};
use std::env;
use std::collections::HashMap;

pub fn get_rocket_config(config_name: &str) -> Result<Config, ConfigError> {
    fn production_config() -> Result<Config, ConfigError> {
        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        Config::build(Environment::Production)
            .address("0.0.0.0")
            .port(8080)
            .extra("environment_name", "production")
            .extra("auth_token_timeout_days", 30)
            .extra("cors_allow_origin", "https://example.com")
            .extra("cors_allow_headers", "*")
            .extra("cors_allow_methods", "*")
            .extra("databases", databases)
            .finalize()
    }

    fn staging_config() -> Result<Config, ConfigError> {
        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(8080)
            .extra("environment_name", "staging")
            .extra("auth_token_timeout_days", 30)
            .extra("cors_allow_origin", "https://staging.example.com")
            .extra("cors_allow_headers", "*")
            .extra("cors_allow_methods", "*")
            .extra("databases", databases)
            .finalize()
    }

    fn develop_config() -> Result<Config, ConfigError> {
        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(8080)
            .extra("environment_name", "develop")
            .extra("auth_token_timeout_days", 30)
            .extra("cors_allow_origin", "https://develop.example.com")
            .extra("cors_allow_headers", "*")
            .extra("cors_allow_methods", "*")
            .extra("databases", databases)
            .finalize()
    }

    fn testing_config() -> Result<Config, ConfigError> {
        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(5000)
            .extra("environment_name", "testing")
            .extra("auth_token_timeout_days", 30)
            .extra("cors_allow_origin", "*")
            .extra("cors_allow_headers", "*")
            .extra("cors_allow_methods", "*")
            .extra("databases", databases)
            .finalize()
    }

    fn local_config() -> Result<Config, ConfigError> {
        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
        databases.insert("postgres_db", Value::from(database_config));

        Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(5000)
            .extra("environment_name", "local")
            .extra("auth_token_timeout_days", 30)
            .extra("cors_allow_origin", "*")
            .extra("cors_allow_headers", "*")
            .extra("cors_allow_methods", "*")
            .extra("databases", databases)
            .finalize()
    }

    match config_name {
        "prod" => production_config(),
        "stage" => staging_config(),
        "dev" => develop_config(),
        "test" => testing_config(),
        "local" => local_config(),
        _ => Err(ConfigError::BadEnv(format!(
            "No valid config chosen: {}",
            config_name
        ))),
    }
}
