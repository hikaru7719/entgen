pub fn convert_to_rs_type(db_type: &str) -> &str {
    // TODO: 一旦varcharのみ対応
    // 対応できる型を増やしていく
    match db_type {
        "uuid" => "sqlx::types::Uuid",
        "varchar" => "String",
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
