#[derive(sqlx::FromRow, Debug)]
pub struct Strs {
    pub aaa: String,
    pub bbb: String,
    pub ccc: String,
    pub ddd: String,
    pub eee: String,
}
