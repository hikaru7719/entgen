use std::{fs, path::Path};

use serde_derive::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub postgres: PostgresConfig,
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i64,
    pub db: String,
}

#[derive(Debug)]
pub enum ConfigError {
    FilePathError(Box<dyn std::error::Error>),
    FileParseError(Box<dyn std::error::Error>),
    EnvError(Box<dyn std::error::Error>),
}

pub fn new(path: String) -> Result<Config, ConfigError> {
    return fs::read_to_string(path)
        .or_else(|err| Err(ConfigError::FilePathError(Box::new(err))))
        .and_then(|path| {
            toml::from_str::<Config>(path.as_str())
                .or_else(|err| Err(ConfigError::FileParseError(Box::new(err))))
        });
}

pub fn new_postgres_config_for_test() -> Result<PostgresConfig, ConfigError> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    dotenv::from_path(path).ok();
    envy::prefixed("POSTGRES_")
        .from_env::<PostgresConfig>()
        .or_else(|err| Err(ConfigError::EnvError(Box::new(err))))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let config = new("test_config.toml".to_string()).unwrap();
        assert_eq!(config.postgres.user, "testuser".to_string());
        assert_eq!(config.postgres.password, "testpassword".to_string());
        assert_eq!(config.postgres.host, "testhost".to_string());
        assert_eq!(config.postgres.port, 5432);
        assert_eq!(config.postgres.db, "testdb".to_string());
    }

    #[test]
    fn test_new_postgres_config_for_test() {
        std::env::set_var("POSTGRES_USER", "testuser");
        std::env::set_var("POSTGRES_PASSWORD", "testpassword");
        std::env::set_var("POSTGRES_HOST", "testhost");
        std::env::set_var("POSTGRES_PORT", "5432");
        std::env::set_var("POSTGRES_DB", "testdb");
        let config = new_postgres_config_for_test().unwrap();
        assert_eq!(config.user, "testuser".to_string());
        assert_eq!(config.password, "testpassword".to_string());
        assert_eq!(config.host, "testhost".to_string());
        assert_eq!(config.port, 5432);
        assert_eq!(config.db, "testdb".to_string());
    }
}
