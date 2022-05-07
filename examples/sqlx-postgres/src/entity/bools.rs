#[derive(sqlx::FromRow, Debug)]
pub struct Bools {
    pub aaa: bool,
}

use std::ops::Deref;
use std::sync::Arc;

pub struct BoolsRepository {
    pool: Arc<sqlx::PgPool>,
}

impl BoolsRepository {
    fn new(pool: Arc<sqlx::PgPool>) -> Self {
        BoolsRepository { pool: pool }
    }

    async fn insert(&self, bools: &Bools) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO bools (aaa) VALUES ($1)")
            .bind(bools.aaa)
            .execute(self.pool.deref())
            .await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Bools>, sqlx::Error> {
        Ok(sqlx::query_as::<_, Bools>("SELECT * FROM bools")
            .fetch_all(self.pool.deref())
            .await?)
    }
}
