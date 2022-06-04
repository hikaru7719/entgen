#[derive(sqlx::FromRow, Debug)]
pub struct Times {
    pub aaa: sqlx::types::chrono::Date,
    pub bbb: sqlx::types::chrono::DateTime<Utc>,
    pub ccc: sqlx::types::chrono::NaiveDateTime,
    pub ddd: sqlx::types::chrono::Time,
    pub eee: sqlx::postgres::types::PgTimeTz,
}

use std::ops::Deref;
use std::sync::Arc;

pub struct TimesRepository {
    pool: Arc<sqlx::PgPool>,
}

impl TimesRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        TimesRepository { pool: pool }
    }

    pub async fn insert(&self, times: &Times) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO times (aaa, bbb, ccc, ddd, eee) VALUES ($1, $2, $3, $4, $5)")
            .bind(times.aaa)
            .bind(times.bbb)
            .bind(times.ccc)
            .bind(times.ddd)
            .bind(times.eee)
            .execute(self.pool.deref())
            .await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Times>, sqlx::Error> {
        Ok(sqlx::query_as::<_, Times>("SELECT * FROM times")
            .fetch_all(self.pool.deref())
            .await?)
    }
}
