#[derive(sqlx::FromRow, Debug)]
pub struct Nums {
    pub aaa: i16,
    pub bbb: i32,
    pub ccc: i64,
    pub ddd: i16,
    pub eee: i32,
    pub fff: i64,
    pub ggg: f32,
    pub hhh: f64,
    pub iii: sqlx::types::Decimal,
    pub jjj: sqlx::types::Decimal,
}

use std::ops::Deref;
use std::sync::Arc;

pub struct NumsRepository {
    pool: Arc<sqlx::PgPool>,
}

impl NumsRepository {
    fn new(pool: Arc<sqlx::PgPool>) -> Self {
        NumsRepository { pool: pool }
    }

    async fn insert(&self, nums: &Nums) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO nums (aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
            .bind(nums.aaa)
            .bind(nums.bbb)
            .bind(nums.ccc)
            .bind(nums.ddd)
            .bind(nums.eee)
            .bind(nums.fff)
            .bind(nums.ggg)
            .bind(nums.hhh)
            .bind(nums.iii)
            .bind(nums.jjj)
            .execute(self.pool.deref())
            .await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Nums>, sqlx::Error> {
        Ok(sqlx::query_as::<_, Nums>("SELECT * FROM nums")
            .fetch_all(self.pool.deref())
            .await?)
    }
}
