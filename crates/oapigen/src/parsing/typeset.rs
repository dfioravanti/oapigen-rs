use crate::parsing::errors::ParsingError;
use crate::parsing::errors::ParsingError::CannotGenerateUnionType;
use crate::parsing::macros::get_macros;
use crate::{format, models, Imports};
use oas3::spec::{ObjectSchema, SchemaType, SchemaTypeSet};
use std::collections::HashSet;

enum BaseType {
    Boolean,
    Integer,
    Number,
    String,
    Null,
}

pub(crate) fn schema_to_rust(
    config: &models::Config,
    inputs: &models::SchemaInputs,
    schema: ObjectSchema,
) -> Result<models::SchemaAsRust, ParsingError> {
    if schema.const_value.is_some() {}

    match &schema.schema_type {
        Some(schema_typeset) => match schema_typeset {
            SchemaTypeSet::Single(single_type) => {
                convert_single_type(config, inputs, &schema, single_type)
            }
            SchemaTypeSet::Multiple(multiple_types) => {
                convert_multiple_types(config, inputs, &schema, multiple_types)
            }
        },
        None => todo!(),
    }
}

fn convert_multiple_types(
    config: &models::Config,
    inputs: &models::SchemaInputs,
    schema: &ObjectSchema,
    schema_types: &Vec<SchemaType>,
) -> Result<models::SchemaAsRust, ParsingError> {
    let mut rust_types: Vec<models::SchemaAsRust> = Vec::with_capacity(schema_types.len());
    for schema_type in schema_types {
        let o = convert_single_type(config, inputs, schema, schema_type)?;
        rust_types.push(o);
    }

    union_type(&rust_types)
}

fn convert_single_type(
    config: &models::Config,
    inputs: &models::SchemaInputs,
    schema: &ObjectSchema,
    schema_type: &SchemaType,
) -> Result<models::SchemaAsRust, ParsingError> {
    match schema_type {
        SchemaType::Null => convert_base_schema_type(config, inputs, BaseType::Null, schema),
        SchemaType::Boolean => convert_base_schema_type(config, inputs, BaseType::Boolean, schema),
        SchemaType::Integer => convert_base_schema_type(config, inputs, BaseType::Integer, schema),
        SchemaType::Number => convert_base_schema_type(config, inputs, BaseType::Number, schema),
        SchemaType::String => convert_base_schema_type(config, inputs, BaseType::String, schema),
        SchemaType::Array => todo!(),
        SchemaType::Object => todo!(),
    }
}

fn convert_base_schema_type(
    config: &models::Config,
    inputs: &models::SchemaInputs,
    schema_type: BaseType,
    schema: &ObjectSchema,
) -> Result<models::SchemaAsRust, ParsingError> {
    let is_optional = match schema_type {
        BaseType::Null => true,
        _ => false,
    };

    let (rust_type, mut imports) = match schema_type {
        BaseType::Integer => match &schema.format {
            Some(format) => format::format_number(format),
            None => format::format_number(format::DEFAULT_INTEGER),
        },
        BaseType::Number => match &schema.format {
            Some(format) => format::format_number(format),
            None => format::format_number(format::DEFAULT_NUMBER),
        },
        BaseType::String => match &schema.format {
            Some(format) => format::format_string(config, format),
            None => format::format_string(config, format::DEFAULT_STRING),
        },
        BaseType::Boolean => format::format_boolean(),
        BaseType::Null => format::format_null(),
    };

    let (tokenized_macros, imports_macros) = get_macros();

    imports.extend(imports_macros);

    Ok(models::SchemaAsRust {
        name: inputs.schema_name.clone(),
        rust_type,
        macros: tokenized_macros,
        imports,
        comment: schema.description.clone(),
        is_optional,
        current_type: models::CurrentType::Type,
    })
}

fn union_type(types: &Vec<models::SchemaAsRust>) -> Result<models::SchemaAsRust, ParsingError> {
    if types.is_empty() {
        return Err(CannotGenerateUnionType("empty list of types".to_string()));
    }

    let mut name = String::new();
    let mut rust_types: Vec<String> = Vec::new();
    let mut macros: HashSet<String> = HashSet::new();
    let mut imports = Imports::new();
    let mut is_optional = false;
    let mut comment: Option<String> = None;

    for t in types {
        if name != "" && name != t.name {
            return Err(CannotGenerateUnionType(format!(
                "all elements of a union type are expected to have the same name, but I got {:} and {:}",
                name, t.name
            )));
        }
        if comment.is_some() && comment != Some("".to_string()) && comment != t.comment {
            return Err(CannotGenerateUnionType(format!(
                "all elements of a union type are expected to have the same name, but I got {:} and {:}",
                name, t.name
            )));
        }

        // todo: think to remove these clones since only the first one is needed
        name = t.name.clone();
        comment = t.comment.clone();
        if t.is_optional {
            is_optional = true;
        } else {
            rust_types.push(t.rust_type.clone());
        }

        macros.extend(t.macros.clone());
        imports.extend(t.imports.clone());
    }

    Ok(models::SchemaAsRust {
        name,
        rust_type: rust_types.join("|"),
        macros,
        imports,
        comment,
        is_optional,
        current_type: models::CurrentType::Type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::{DateTimeLibraries, Libraries};
    use rstest::rstest;

    #[rstest]
    #[case("integer", "Age", "{type: integer, description: The requested date}")]
    #[case("number", "Height", "type: number")]
    #[case("string", "Name", "type: string")]
    #[case("boolean", "True", "type: boolean")]
    fn test_parse_base_cases(
        #[case] name: &str,
        #[case] schema_name: &str,
        #[case] schema_spec: &str,
    ) {
        let config = models::Config {
            output_path: Default::default(),
            libraries: Libraries {
                datetime: DateTimeLibraries::Chrono,
            },
        };

        let mut insta_settings = insta::Settings::clone_current();
        insta_settings.set_snapshot_suffix(name);

        let schema = serde_yaml::from_str::<ObjectSchema>(schema_spec).unwrap();

        let inputs = models::SchemaInputs {
            schema_name: &schema_name.to_string(),
        };
        let got = schema_to_rust(&config, &inputs, schema).unwrap();

        insta_settings.bind(|| {
            insta::assert_snapshot!(got.to_string());
        });
    }

    /*

    TODO: fix constant test
       #[rstest]
       #[case("integer", "Age", "const: 3", "type Age = i32 ;")]
       fn test_parse_constants(
           #[case] name: &str,
           #[case] schema_name: &str,
           #[case] schema_spec: &str,
           #[case] expected: &str,
       ) {
           let mut settings = insta::Settings::clone_current();
           settings.set_snapshot_suffix(name);

           let schema = serde_yaml::from_str::<ObjectSchema>(schema_spec).unwrap();

           let got = tokenize_schema(schema_name.to_string(), schema);

           settings.bind(|| {
               insta::assert_yaml_snapshot!(got.to_string());
           });
           assert_eq!(got.to_string(), expected.to_string());
       }

    */
}
