use crate::models;
use crate::templates::init;
use anyhow::{Context, Result};
use handlebars::Handlebars;

fn parse_struct(struct_to_be_parsed: models::Struct, handlebars: Handlebars) -> Result<String> {
    let result = handlebars
        .render(init::STRUCT_TEMPLATE_NAME, &struct_to_be_parsed)
        .with_context(|| {
            format!(
                "failed to render struct {:?}",
                struct_to_be_parsed.struct_name
            )
        })?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta;
    #[test]
    fn test_struct_is_rendered_correctly() {
        let handlebars = init::handlebars().unwrap();

        let input = models::Struct {
            struct_name: "User".to_string(),
            fields: vec![
                models::Field {
                    field_name: "name".to_string(),
                    field_type: "String".to_string(),
                    comment: None,
                    annotations: vec![],
                },
                models::Field {
                    field_name: "lastname".to_string(),
                    field_type: "String".to_string(),
                    comment: None,
                    annotations: vec![],
                },
            ],
        };

        let got = parse_struct(input, handlebars).unwrap();
        insta::assert_yaml_snapshot!(got);
    }
}
