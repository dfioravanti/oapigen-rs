use crate::parsing::errors::ParsingError;
use crate::{format, models};
use oas3::spec::{ObjectSchema, SchemaType, SchemaTypeSet};
use proc_macro2::TokenStream;

pub(crate) fn tokenize_schema(
    config: &models::Config,
    schema_name: String,
    schema: ObjectSchema,
) -> Result<models::SchemaAsRust, ParsingError> {
    if schema.const_value.is_some() {}

    let tokenized_schema = match &schema.schema_type {
        Some(schema_typeset) => match schema_typeset {
            SchemaTypeSet::Single(schema_type) => {
                tokenize_type(config, schema_name, &schema, schema_type)
            }
            SchemaTypeSet::Multiple(_items) => todo!(),
        },
        None => todo!(),
    };

    tokenized_schema
}

fn tokenize_type(
    config: &models::Config,
    schema_name: String,
    schema: &ObjectSchema,
    schema_type: &SchemaType,
) -> Result<models::SchemaAsRust, ParsingError> {
    match schema_type {
        SchemaType::Boolean => todo!(),
        SchemaType::Integer => tokenize_integer_schema(schema_name, schema),
        SchemaType::Number => tokenize_number_schema(schema_name, schema),
        SchemaType::String => tokenize_string_schema(config, schema_name, schema),
        SchemaType::Array => todo!(),
        SchemaType::Object => todo!(),
        SchemaType::Null => todo!(),
    }
}

fn tokenize_integer_schema(
    schema_name: String,
    schema: &ObjectSchema,
) -> Result<models::SchemaAsRust, ParsingError> {
    // if one injects schema_name directly into quote it gets tokenized together with "",
    // which we do not want. Doing it like this drops the ""
    let tokenized_name: TokenStream = schema_name.parse()?;

    let (tokenized_type, imports) = match &schema.format {
        Some(format) => format::format_number(format),
        None => format::format_number(format::DEFAULT_INTEGER),
    };

    Ok(models::SchemaAsRust {
        tokenized_name,
        tokenized_type,
        imports,
        current_type: models::CurrentType::Type,
    })
}

fn tokenize_number_schema(
    schema_name: String,
    schema: &ObjectSchema,
) -> Result<models::SchemaAsRust, ParsingError> {
    // if one injects schema_name directly into quote it gets tokenized together with "",
    // which we do not want. Doing it like this drops the ""
    let tokenized_name: TokenStream = schema_name.parse()?;

    let (tokenized_type, imports) = match &schema.format {
        Some(format) => format::format_number(format),
        None => format::format_number(format::DEFAULT_NUMBER),
    };

    Ok(models::SchemaAsRust {
        tokenized_name,
        tokenized_type,
        imports,
        current_type: models::CurrentType::Type,
    })
}

fn tokenize_string_schema(
    config: &models::Config,
    schema_name: String,
    schema: &ObjectSchema,
) -> Result<models::SchemaAsRust, ParsingError> {
    // if one injects schema_name directly into quote it gets tokenized together with "",
    // which we do not want. Doing it like this drops the ""
    let tokenized_name: TokenStream = schema_name.parse()?;

    let (tokenized_type, imports) = match &schema.format {
        Some(format) => format::format_string(config, format),
        None => format::format_string(config, format::DEFAULT_STRING),
    };

    Ok(models::SchemaAsRust {
        tokenized_name,
        tokenized_type,
        imports,
        current_type: models::CurrentType::Type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::{DateTimeLibraries, Libraries};
    use rstest::rstest;

    #[rstest]
    #[case("integer", "Age", "type: integer")]
    #[case("number", "Height", "type: number")]
    #[case("string", "Name", "type: string")]
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

        let got = tokenize_schema(&config, schema_name.to_string(), schema).unwrap();

        insta_settings.bind(|| {
            insta::assert_yaml_snapshot!(got.to_string());
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
