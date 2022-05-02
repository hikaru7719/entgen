use askama::Template;

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
}

impl EntityTemplate {
    pub fn build(self) -> Result<String, TemplateError> {
        self.render()
            .or_else(|err| Err(TemplateError::BuildFailed(err.into())))
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
}"#
            .to_string()
        );
    }
}
