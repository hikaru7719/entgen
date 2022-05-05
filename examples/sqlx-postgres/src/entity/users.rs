#[derive(sqlx::FromRow, Debug)]
pub struct Users {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub nickname: Option<String>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}
