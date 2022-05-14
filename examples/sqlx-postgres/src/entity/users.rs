#[derive(sqlx::FromRow, Debug)]
pub struct Users {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub nickname: Option<String>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

use std::ops::Deref;
use std::sync::Arc;

pub struct UsersRepository {
    pool: Arc<sqlx::PgPool>,
}

impl UsersRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        UsersRepository { pool: pool }
    }

    pub async fn insert(&self, users: &Users) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (id, name, nickname, created_at) VALUES ($1, $2, $3, $4)")
            .bind(users.id)
            .bind(users.name.clone())
            .bind(users.nickname.as_ref())
            .bind(users.created_at)
            .execute(self.pool.deref())
            .await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Users>, sqlx::Error> {
        Ok(sqlx::query_as::<_, Users>("SELECT * FROM users")
            .fetch_all(self.pool.deref())
            .await?)
    }

    pub async fn find_by_id(&self, id: &sqlx::types::Uuid) -> Result<Users, sqlx::Error> {
        Ok(
            sqlx::query_as::<_, Users>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(self.pool.deref())
                .await?,
        )
    }

    pub async fn delete(&self, id: &sqlx::types::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(self.pool.deref())
            .await?;
        Ok(())
    }
}
