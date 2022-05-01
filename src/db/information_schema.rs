use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config;

#[derive(sqlx::FromRow, Debug)]
pub struct Tables {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_type: String,
    pub self_referencing_column_name: Option<String>,
    pub reference_generation: Option<String>,
    pub user_defined_type_catalog: String,
    pub user_defined_type_schema: String,
    pub user_defined_type_name: String,
    pub is_insertable_into: String,
    pub is_typed: String,
    pub commit_action: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Columns {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub ordinal_position: String,
    pub column_default: String,
    pub is_nullable: String,
    pub data_type: String,
    pub character_maximum_length: i64,
    pub character_octet_length: i64,
    pub numeric_precision: i64,
    pub numeric_precision_radix: i64,
    pub numeric_scale: i64,
    pub datetime_precision: i64,
    pub interval_type: String,
    pub interval_precision: String,
    pub character_set_catalog: String,
    pub character_set_schema: String,
    pub character_set_name: String,
    pub collation_catalog: String,
    pub collation_schema: String,
    pub collation_name: String,
    pub domain_catalog: String,
    pub domain_schema: String,
    pub domain_name: String,
    pub udt_catalog: String,
    pub udt_schema: String,
    pub udt_name: String,
    pub scope_catalog: String,
    pub scope_schema: String,
    pub scope_name: String,
    pub maximum_cardinality: String,
    pub dtd_identifier: String,
    pub is_self_referencing: String,
    pub is_identity: String,
    pub identity_generation: String,
    pub identity_start: String,
    pub identity_increment: String,
    pub identity_maximum: String,
    pub identity_minimum: String,
    pub identity_cycle: String,
    pub is_generated: String,
    pub generation_expression: String,
    pub is_updatable: String,
}

#[derive(Debug)]
pub enum DBError {
    ConnectionError(Box<dyn std::error::Error>),
    QueryError(Box<dyn std::error::Error>),
}

pub async fn connect(config: config::PostgresConfig) -> Result<PgPool, DBError> {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.db
    );
    PgPoolOptions::new()
        .connect(db_url.as_str())
        .await
        .or_else(|err| Err(DBError::ConnectionError(err.into())))
}

pub async fn close(pool: &PgPool) {
    pool.close().await;
}

pub async fn fetch_user_defined_tables(pool: &PgPool) -> Result<Vec<String>, DBError> {
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
        .or_else(|err| Err(DBError::QueryError(err.into())));
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let config = config::new_postgres_config_for_test().unwrap();
        let pool = connect(config).await.unwrap();
        assert_eq!(pool.is_closed(), false);
        close(&pool).await;
        assert_eq!(pool.is_closed(), true);
    }

    #[tokio::test]
    async fn test_fetch_user_defined_tables() {
        let config = config::new_postgres_config_for_test().unwrap();
        let pool = connect(config).await.unwrap();
        let tables = fetch_user_defined_tables(&pool).await.unwrap();
        assert_eq!(vec!["users"], tables);
    }
}
