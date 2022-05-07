use crate::error::EntgenError;

use crate::db::{converter, information_schema};
use askama::Template;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

#[derive(Template, Debug)]
#[template(path = "entity_template.txt")]
pub struct EntityTemplate {
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
    db_models: Vec<information_schema::Columns>,
) -> EntityTemplate {
    let fields: Vec<Field> = db_models
        .into_iter()
        .map(|db_model| from(db_model))
        .collect();
    EntityTemplate {
        entity_name: table_name.clone(),
        fields: fields,
    }
}

pub fn from(db_model: information_schema::Columns) -> Field {
    let field_name = db_model.column_name.clone();
    let field_type = converter::convert_to_rs_type(db_model.udt_name.as_str()).to_string();
    let nullable = converter::convert_to_bool(db_model.is_nullable.as_str());
    Field {
        field_name: field_name,
        field_type: field_type,
        nullable: nullable,
    }
}

impl EntityTemplate {
    pub fn build(&self) -> Result<String, EntgenError> {
        self.render()
            .and_then(|s| Ok(format!("{}\n", s)))
            .or_else(|err| Err(EntgenError::TemplateBuildFailed(err.into())))
    }

    pub fn write(&self, output_dir: &String, body: String) -> Result<(), EntgenError> {
        let dir = Path::new(&output_dir);
        let path = dir.join(format!("{}.rs", self.entity_name));
        let mut file: File;

        if dir.exists() && path.exists() {
            file = File::open(path)
                .or_else(|err| Err(EntgenError::TemplateFileOpenError(Box::new(err))))?;
        } else if dir.exists() {
            file = File::create(path)
                .or_else(|err| Err(EntgenError::TemplateFileOpenError(Box::new(err))))?;
        } else {
            create_dir_all(dir)
                .or_else(|err| Err(EntgenError::TemplateDirCreateError(Box::new(err))))?;

            file = File::create(path)
                .or_else(|err| Err(EntgenError::TemplateFileOpenError(Box::new(err))))?;
        }

        file.write(body.as_bytes())
            .or_else(|err| Err(EntgenError::TemplateFileWriteError(Box::new(err))))?;
        Ok(())
    }

    pub fn build_and_write(&self, output_dir: &String) -> Result<(), EntgenError> {
        let body = self.build()?;
        self.write(output_dir, body)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build() {
        let template = EntityTemplate {
            entity_name: "entity".to_string(),
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
            r#"#[derive(sqlx::FromRow, Debug)]
pub struct Entity {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub nickname: Option<String>,
}
"#
            .to_string()
        );
    }
}
