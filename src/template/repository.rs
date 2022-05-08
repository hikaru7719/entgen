use crate::error::EntgenError;

use crate::db::{converter, information_schema};
use askama::Template;

#[derive(Template, Debug)]
#[template(path = "repository_template.txt")]
pub struct RepositoryTemplate {
    entity_name: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    field_name: String,
    field_type: String,
    nullable: bool,
}

pub fn from_vec(
    table_name: &String,
    db_models: &Vec<information_schema::Columns>,
) -> RepositoryTemplate {
    let fields: Vec<Field> = db_models.iter().map(|db_model| from(db_model)).collect();
    RepositoryTemplate {
        entity_name: table_name.clone(),
        fields: fields,
    }
}

pub fn from(db_model: &information_schema::Columns) -> Field {
    let field_name = db_model.column_name.clone();
    let field_type = converter::convert_to_rs_type(db_model.udt_name.as_str()).to_string();
    let nullable = converter::convert_to_bool(db_model.is_nullable.as_str());
    Field {
        field_name: field_name,
        field_type: field_type,
        nullable: nullable,
    }
}

impl RepositoryTemplate {
    pub fn build(&self) -> Result<String, EntgenError> {
        self.render()
            .and_then(|s| Ok(format!("{}\n", s)))
            .or_else(|err| Err(EntgenError::TemplateBuildFailed(err.into())))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build() {
        let template = RepositoryTemplate {
            entity_name: "users".to_string(),
            fields: vec![
                Field {
                    field_name: "id".to_string(),
                    field_type: "sqlx::types::Uuid".to_string(),
                    nullable: false,
                },
                Field {
                    field_name: "name".to_string(),
                    field_type: "String".to_string(),
                    nullable: false,
                },
                Field {
                    field_name: "nickname".to_string(),
                    field_type: "String".to_string(),
                    nullable: true,
                },
            ],
        };
        assert_eq!(
            template.build().unwrap(),
            r#"use std::ops::Deref;
use std::sync::Arc;

pub struct UsersRepository {
    pool: Arc<sqlx::PgPool>,
}

impl UsersRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        UsersRepository { pool: pool }
    }

    pub async fn insert(&self, users: &Users) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (id, name, nickname) VALUES ($1, $2, $3)")
            .bind(users.id)
            .bind(users.name.clone())
            .bind(users.nickname.as_ref())
            .execute(self.pool.deref())
            .await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Users>, sqlx::Error> {
        Ok(sqlx::query_as::<_, Users>("SELECT * FROM users")
            .fetch_all(self.pool.deref())
            .await?)
    }
}
"#
            .to_string()
        );
    }
}
