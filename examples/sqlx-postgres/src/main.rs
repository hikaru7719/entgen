mod entity;

use std::{str::FromStr, sync::Arc};

use entity::{nums::Nums, users::Users};
use sqlx::{postgres::PgPoolOptions, types::Uuid};

use crate::entity::{nums::NumsRepository, users::UsersRepository};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://testuser:testpassword@localhost:5432/testdb")
            .await?,
    );
    let users = Users {
        id: Uuid::new_v4(),
        name: "testuser".to_string(),
        nickname: Some("test".to_string()),
        created_at: sqlx::types::chrono::NaiveDateTime::new(
            sqlx::types::chrono::NaiveDate::from_ymd(2022, 5, 5),
            sqlx::types::chrono::NaiveTime::from_hms_milli(0, 0, 0, 0),
        ),
    };

    let users_repository = UsersRepository::new(pool.clone());
    users_repository.insert(&users).await.unwrap();
    let all_users = users_repository.find_all().await.unwrap();
    println!("{:?}", all_users);

    let nums = Nums {
        aaa: 1i16,
        bbb: 1i32,
        ccc: 1i64,
        ddd: 1i16,
        eee: 1i32,
        fff: 1i64,
        ggg: 1f32,
        hhh: 1f64,
        iii: sqlx::types::Decimal::from_str("2.1").unwrap(),
        jjj: sqlx::types::Decimal::from_str("2.1").unwrap(),
    };

    let nums_repository = NumsRepository::new(pool.clone());
    nums_repository.insert(&nums).await.unwrap();
    let all_nums = nums_repository.find_all().await.unwrap();
    println!("{:?}", all_nums);

    Ok(())
}
