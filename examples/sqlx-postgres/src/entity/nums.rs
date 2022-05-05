#[derive(sqlx::FromRow, Debug)]
pub struct Nums {
    pub aaa: i16,
    pub bbb: i32,
    pub ccc: i64,
    pub ddd: i16,
    pub eee: i32,
    pub fff: i64,
}
