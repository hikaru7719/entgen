#[derive(sqlx::FromRow, Debug)]
pub struct Strs {
    pub aaa: String,
    pub bbb: String,
    pub ccc: String,
    pub ddd: String,
    pub eee: String,
}

use std::ops::Deref;
use std::sync::Arc;

pub struct StrsRepository {
    pool: Arc<sqlx::PgPool>,
}

impl StrsRepository {
    fn new(pool: Arc<sqlx::PgPool>) -> Self {
        StrsRepository { pool: pool }
    }

    async fn insert(&self, strs: &Strs) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO strs (aaa, bbb, ccc, ddd, eee) VALUES ($1, $2, $3, $4, $5)")
            .bind(strs.aaa.clone())
            .bind(strs.bbb.clone())
            .bind(strs.ccc.clone())
            .bind(strs.ddd.clone())
            .bind(strs.eee.clone())
            .execute(self.pool.deref())
            .await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Strs>, sqlx::Error> {
        Ok(sqlx::query_as::<_, Strs>("SELECT * FROM strs")
            .fetch_all(self.pool.deref())
            .await?)
    }
}
