mod entity;

use std::str::FromStr;

use entity::{nums::Nums, users::Users};
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

    insert_user(&pool, &user).await.unwrap();
    let user2 = select_user(&pool, &user.id).await.unwrap();

    assert_eq!(user.id.to_string(), user2.id.to_string());
    assert_eq!(user.name, user2.name);
    assert_eq!(user.nickname, user2.nickname);
    println!("{:?}", user2);

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
    insert_nums(&pool, &nums).await.unwrap();
    let nums2 = select_nums(&pool).await.unwrap();
    println!("{:?}", nums2);

    assert_eq!(nums.aaa, nums2.aaa);
    assert_eq!(nums.bbb, nums2.bbb);
    assert_eq!(nums.ccc, nums2.ccc);
    assert_eq!(nums.ddd, nums2.ddd);
    assert_eq!(nums.eee, nums2.eee);
    assert_eq!(nums.fff, nums2.fff);
    assert_eq!(nums.ggg, nums2.ggg);
    assert_eq!(nums.hhh, nums2.hhh);
    assert_eq!(nums.iii, nums2.iii);
    assert_eq!(nums.jjj, nums2.jjj);

    Ok(())
}

async fn insert_user(pool: &PgPool, user: &Users) -> Result<(), sqlx::Error> {
    sqlx::query("insert into users (id, name, nickname, created_at) values ($1, $2, $3, $4)")
        .bind(user.id.clone())
        .bind(user.name.clone())
        .bind(user.nickname.as_ref())
        .bind(user.created_at)
        .execute(pool)
        .await?;
    Ok(())
}

async fn select_user(pool: &PgPool, id: &Uuid) -> Result<Users, sqlx::Error> {
    Ok(
        sqlx::query_as::<_, Users>("select * from users where id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?,
    )
}

async fn insert_nums(pool: &PgPool, nums: &Nums) -> Result<(), sqlx::Error> {
    sqlx::query(
        "insert into nums (aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
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
    .execute(pool)
    .await?;
    Ok(())
}

async fn select_nums(pool: &PgPool) -> Result<Nums, sqlx::Error> {
    Ok(sqlx::query_as::<_, Nums>("select * from nums limit 1")
        .fetch_one(pool)
        .await?)
}
