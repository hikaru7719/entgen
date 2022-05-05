use super::db::{converter, information_schema};
use askama::Template;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

#[derive(Template, Debug)]
#[template(path = "default_template.txt")]
pub struct EntityTemplate {
    entity_name: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    field_name: String,
    field_type: String,
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
    let field_type =
        converter::convert_to_field_type(db_model.udt_name.as_str(), db_model.is_nullable.as_str());
    Field {
        field_name: field_name,
        field_type: field_type,
    }
}

#[derive(Debug)]
pub enum TemplateError {
    BuildFailed(Box<dyn std::error::Error>),
    FileOpenError(Box<dyn std::error::Error>),
    FileWriteError(Box<dyn std::error::Error>),
    DirCreateError(Box<dyn std::error::Error>),
}

impl EntityTemplate {
    pub fn build(&self) -> Result<String, TemplateError> {
        self.render()
            .and_then(|s| Ok(format!("{}\n", s)))
            .or_else(|err| Err(TemplateError::BuildFailed(err.into())))
    }

    pub fn write(&self, output_dir: String, body: String) -> Result<(), TemplateError> {
        let dir = Path::new(&output_dir);
        let path = dir.join(format!("{}.generated.rs", self.entity_name));
        let mut file: File;

        if dir.exists() && path.exists() {
            file =
                File::open(path).or_else(|err| Err(TemplateError::FileOpenError(Box::new(err))))?;
        } else if dir.exists() {
            file = File::create(path)
                .or_else(|err| Err(TemplateError::FileOpenError(Box::new(err))))?;
        } else {
            create_dir_all(dir).or_else(|err| Err(TemplateError::DirCreateError(Box::new(err))))?;

            file = File::create(path)
                .or_else(|err| Err(TemplateError::FileOpenError(Box::new(err))))?;
        }

        file.write(body.as_bytes())
            .or_else(|err| Err(TemplateError::FileWriteError(Box::new(err))))?;
        Ok(())
    }

    pub fn build_and_write(&self, output_dir: String) -> Result<(), TemplateError> {
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
            entity_name: "TestEntity".to_string(),
            fields: vec![
                Field {
                    field_name: "id".to_string(),
                    field_type: "sqlx::types::Uuid".to_string(),
                },
                Field {
                    field_name: "name".to_string(),
                    field_type: "String".to_string(),
                },
            ],
        };
        assert_eq!(
            template.build().unwrap(),
            r#"#[derive(sqlx::FromRow, Debug)]
pub struct TestEntity {
    id: sqlx::types::Uuid,
    name: String,
}
"#
            .to_string()
        );
    }
}
