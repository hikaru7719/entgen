use serde_derive::Deserialize;
use std::{fs, path::PathBuf};

use crate::error::EntgenError;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub output_dir: String,
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

impl Config {
    pub fn new(path: PathBuf) -> Result<Config, EntgenError> {
        return fs::read_to_string(path)
            .or_else(|err| Err(EntgenError::ConfigFilePathError(Box::new(err))))
            .and_then(|path| {
                toml::from_str::<Config>(path.as_str())
                    .or_else(|err| Err(EntgenError::ConfigFileParseError(Box::new(err))))
            });
    }

    pub fn new_postgres_config_for_test() -> PostgresConfig {
        let config = Config::new(PathBuf::from("entgen.toml")).unwrap();
        config.postgres
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let config = Config::new(PathBuf::from("entgen.toml")).unwrap();
        assert_eq!(config.output_dir, "test".to_string());
        assert_eq!(config.postgres.user, "testuser".to_string());
        assert_eq!(config.postgres.password, "testpassword".to_string());
        assert_eq!(config.postgres.host, "localhost".to_string());
        assert_eq!(config.postgres.port, 5432);
        assert_eq!(config.postgres.db, "testdb".to_string());
    }

    #[test]
    fn test_new_postgres_config_for_test() {
        let config = Config::new_postgres_config_for_test();
        assert!(config.user != "".to_string());
        assert!(config.password != "".to_string());
        assert!(config.host != "".to_string());
        assert!(config.port != 0);
        assert!(config.db != "".to_string());
    }
}
