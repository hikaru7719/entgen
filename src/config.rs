use std::fs;

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
}

pub fn new(path: String) -> Result<Config, ConfigError> {
    return fs::read_to_string(path)
        .or_else(|err| Err(ConfigError::FilePathError(Box::new(err))))
        .and_then(|path| {
            toml::from_str::<Config>(path.as_str())
                .or_else(|err| Err(ConfigError::FileParseError(Box::new(err))))
        });
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_config_new() {
        let config = new("test_config.toml".to_string()).unwrap();
        assert_eq!(config.postgres.user, "testuser".to_string());
        assert_eq!(config.postgres.password, "testpassword".to_string());
        assert_eq!(config.postgres.host, "testhost".to_string());
        assert_eq!(config.postgres.port, 5432);
        assert_eq!(config.postgres.db, "testdb".to_string());
    }
}
