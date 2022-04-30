use sqlx::{postgres::PgPoolOptions, types::Uuid, PgPool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let p_user = std::env::var("POSTGRES_USER").unwrap();
    let p_pass = std::env::var("POSTGRES_PASSWORD").unwrap();
    let p_db = std::env::var("POSTGRES_DB").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(format!("postgres://{}:{}@localhost:5432/{}", p_user, p_pass, p_db).as_str())
        .await?;

    let user = User {
        id: Uuid::new_v4(),
        name: "testuser".to_string(),
    };

    insert(&pool, &user).await.unwrap();
    let user2 = select(&pool, &user.id).await.unwrap();

    assert_eq!(user.id.to_string(), user2.id.to_string());
    assert_eq!(user.name, user2.name);
    println!("{:?}", user2);
    Ok(())
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    id: Uuid,
    name: String,
}

async fn insert(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query("insert into users (id, name) values ($1, $2)")
        .bind(user.id.clone())
        .bind(user.name.clone())
        .execute(pool)
        .await?;
    Ok(())
}

async fn select(pool: &PgPool, id: &Uuid) -> Result<User, sqlx::Error> {
    Ok(
        sqlx::query_as::<_, User>("select * from users where id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?,
    )
}
