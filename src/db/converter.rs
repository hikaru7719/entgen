pub fn convert_to_rs_type(db_type: &str) -> &str {
    // TODO: 対応できる型を増やしていく
    match db_type {
        "uuid" => "sqlx::types::Uuid",
        "varchar" => "String",
        "text" => "String",
        "timestamp" => "sqlx::types::chrono::NaiveDateTime",
        "boolean" => "bool",
        "int2" => "i16",
        "int4" => "i32",
        "int8" => "i64",
        "float4" => "f32",
        "float8" => "f64",
        "numeric" => "sqlx::types::Decimal",
        _ => "",
    }
}

pub fn convert_to_field_type(db_type: &str, nullable: &str) -> String {
    let rs_type = convert_to_rs_type(db_type);
    if nullable == "YES" {
        format!("Option<{}>", rs_type)
    } else if nullable == "No" {
        rs_type.to_string()
    } else {
        rs_type.to_string()
    }
}
