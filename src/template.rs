use askama::Template;
use std::{fs::File, io::Write, path::Path};

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

#[derive(Debug)]
pub enum TemplateError {
    BuildFailed(Box<dyn std::error::Error>),
    FileOpenError(Box<dyn std::error::Error>),
    FileWriteError(Box<dyn std::error::Error>),
}

impl EntityTemplate {
    pub fn build(&self) -> Result<String, TemplateError> {
        self.render()
            .and_then(|s| Ok(format!("{}\n", s)))
            .or_else(|err| Err(TemplateError::BuildFailed(err.into())))
    }

    pub fn write(&self, output_dir: String, body: String) -> Result<(), TemplateError> {
        let path = Path::new(&output_dir).join(format!("{}.generated.rs", self.entity_name));

        let mut file: File;

        if path.exists() {
            file =
                File::open(path).or_else(|err| Err(TemplateError::FileOpenError(Box::new(err))))?;
        } else {
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
