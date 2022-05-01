use super::information_schema;

pub struct RsModel {
    pub field_name: String,
    pub field_type: String,
    pub optional: bool,
}

pub fn from_vec(db_models: Vec<information_schema::Columns>) -> Vec<RsModel> {
    db_models
        .into_iter()
        .map(|db_model| from(db_model))
        .collect()
}

pub fn from(db_model: information_schema::Columns) -> RsModel {
    let field_name = db_model.column_name.clone();
    let field_type = convert_to_rs_type(db_model.udt_name.as_str()).to_string();
    let optional = convert_to_optional(db_model.is_nullable.as_str());
    RsModel {
        field_name: field_name,
        field_type: field_type,
        optional: optional,
    }
}

pub fn convert_to_rs_type(db_type: &str) -> &str {
    // TODO: 一旦varcharのみ対応
    // 対応できる型を増やしていく
    match db_type {
        "uuid" => "sqlx::types::Uuid",
        "varchar" => "String",
        _ => "",
    }
}

pub fn convert_to_optional(nullable: &str) -> bool {
    if nullable == "YES" {
        true
    } else if nullable == "No" {
        false
    } else {
        false
    }
}
