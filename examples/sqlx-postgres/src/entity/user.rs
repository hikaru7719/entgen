use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}
