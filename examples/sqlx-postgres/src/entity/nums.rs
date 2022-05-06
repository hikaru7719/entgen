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
