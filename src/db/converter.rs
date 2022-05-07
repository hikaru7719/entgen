pub fn convert_to_rs_type(db_type: &str) -> &str {
    // TODO: 対応できる型を増やしていく
    match db_type {
        "uuid" => "sqlx::types::Uuid",
        "varchar" => "String",
        "text" => "String",
        "bpchar" => "String",
        "name" => "String",
        "timestamp" => "sqlx::types::chrono::NaiveDateTime",
        "bool" => "bool",
        "int2" => "i16",
        "int4" => "i32",
        "int8" => "i64",
        "float4" => "f32",
        "float8" => "f64",
        "numeric" => "sqlx::types::Decimal",
        _ => "",
    }
}

pub fn convert_to_bool(nullable: &str) -> bool {
    if nullable == "YES" {
        true
    } else if nullable == "No" {
        false
    } else {
        false
    }
}
