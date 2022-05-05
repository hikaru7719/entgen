use crate::{config, error::EntgenError};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(sqlx::FromRow, Debug)]
pub struct Tables {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_type: String,
    pub self_referencing_column_name: Option<String>,
    pub reference_generation: Option<String>,
    pub user_defined_type_catalog: Option<String>,
    pub user_defined_type_schema: Option<String>,
    pub user_defined_type_name: Option<String>,
    pub is_insertable_into: String,
    pub is_typed: String,
    pub commit_action: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Columns {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub ordinal_position: i32,
    pub column_default: Option<String>,
    pub is_nullable: String,
    pub data_type: String,
    pub character_maximum_length: Option<i32>,
    pub character_octet_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_precision_radix: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub datetime_precision: Option<i32>,
    pub interval_type: Option<String>,
    pub interval_precision: Option<String>,
    pub character_set_catalog: Option<String>,
    pub character_set_schema: Option<String>,
    pub character_set_name: Option<String>,
    pub collation_catalog: Option<String>,
    pub collation_schema: Option<String>,
    pub collation_name: Option<String>,
    pub domain_catalog: Option<String>,
    pub domain_schema: Option<String>,
    pub domain_name: Option<String>,
    pub udt_catalog: String,
    pub udt_schema: String,
    pub udt_name: String,
    pub scope_catalog: Option<String>,
    pub scope_schema: Option<String>,
    pub scope_name: Option<String>,
    pub maximum_cardinality: Option<String>,
    pub dtd_identifier: String,
    pub is_self_referencing: String,
    pub is_identity: String,
    pub identity_generation: Option<String>,
    pub identity_start: Option<String>,
    pub identity_increment: Option<String>,
    pub identity_maximum: Option<String>,
    pub identity_minimum: Option<String>,
    pub identity_cycle: Option<String>,
    pub is_generated: String,
    pub generation_expression: Option<String>,
    pub is_updatable: String,
}

pub type Pool = PgPool;

pub async fn connect(config: &config::PostgresConfig) -> Result<Pool, EntgenError> {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.db
    );
    PgPoolOptions::new()
        .connect(db_url.as_str())
        .await
        .or_else(|err| Err(EntgenError::DBConnectionError(err.into())))
}

pub async fn close(pool: &Pool) {
    pool.close().await;
}

pub async fn fetch_user_defined_tables(pool: &PgPool) -> Result<Vec<String>, EntgenError> {
    let rs = sqlx::query_as::<_, Tables>(
        "SELECT * FROM information_schema.tables WHERE table_schema = 'public'",
    )
    .fetch_all(pool)
    .await;

    return rs
        .and_then(|tables| {
            return Ok(tables
                .iter()
                .map(|tbl| tbl.table_name.clone())
                .collect::<Vec<String>>());
        })
        .or_else(|err| Err(EntgenError::DBQueryError(err.into())));
}

pub async fn fetch_column_definition(
    pool: &PgPool,
    table_name: &String,
) -> Result<Vec<Columns>, EntgenError> {
    sqlx::query_as::<_, Columns>(
        "SELECT * FROM information_schema.columns WHERE table_name = $1 ORDER BY ordinal_position",
    )
    .bind(table_name)
    .fetch_all(pool)
    .await
    .or_else(|err| Err(EntgenError::DBQueryError(err.into())))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let config = config::Config::new_postgres_config_for_test().unwrap();
        let pool = connect(&config).await.unwrap();
        assert_eq!(pool.is_closed(), false);
        close(&pool).await;
        assert_eq!(pool.is_closed(), true);
    }

    #[tokio::test]
    async fn test_fetch_user_defined_tables() {
        let config = config::Config::new_postgres_config_for_test().unwrap();
        let pool = connect(&config).await.unwrap();
        let tables = fetch_user_defined_tables(&pool).await.unwrap();
        assert_eq!(vec!["users"], tables);
    }

    #[tokio::test]
    async fn test_fetch_column_definition() {
        let config = config::Config::new_postgres_config_for_test().unwrap();
        let pool = connect(&config).await.unwrap();
        let definitions = fetch_column_definition(&pool, &"users".to_string())
            .await
            .unwrap();
        assert_eq!("id".to_string(), definitions[0].column_name);
    }
}
