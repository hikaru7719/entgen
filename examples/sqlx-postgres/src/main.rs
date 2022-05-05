mod entity;

use entity::users::Users;
use sqlx::{postgres::PgPoolOptions, types::Uuid, PgPool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://testuser:testpassword@localhost:5432/testdb")
        .await?;

    let user = Users {
        id: Uuid::new_v4(),
        name: "testuser".to_string(),
        nickname: Some("test".to_string()),
        created_at: sqlx::types::chrono::NaiveDateTime::new(
            sqlx::types::chrono::NaiveDate::from_ymd(2022, 5, 5),
            sqlx::types::chrono::NaiveTime::from_hms_milli(0, 0, 0, 0),
        ),
    };

    insert(&pool, &user).await.unwrap();
    let user2 = select(&pool, &user.id).await.unwrap();

    assert_eq!(user.id.to_string(), user2.id.to_string());
    assert_eq!(user.name, user2.name);
    assert_eq!(user.nickname, user2.nickname);
    println!("{:?}", user2);
    Ok(())
}

async fn insert(pool: &PgPool, user: &Users) -> Result<(), sqlx::Error> {
    sqlx::query("insert into users (id, name, nickname, created_at) values ($1, $2, $3, $4)")
        .bind(user.id.clone())
        .bind(user.name.clone())
        .bind(user.nickname.as_ref())
        .bind(user.created_at)
        .execute(pool)
        .await?;
    Ok(())
}

async fn select(pool: &PgPool, id: &Uuid) -> Result<Users, sqlx::Error> {
    Ok(
        sqlx::query_as::<_, Users>("select * from users where id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?,
    )
}
