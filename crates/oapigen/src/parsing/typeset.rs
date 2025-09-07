use crate::{models, parsing::schema_format};
use oas3::spec::{ObjectSchema, SchemaType, SchemaTypeSet};
use proc_macro2::TokenStream;
use quote::ToTokens;

pub fn tokenize_schema(schema_name: String, schema: ObjectSchema) -> TokenStream {
    if schema.const_value.is_some() {}

    let tokenized_schema = match &schema.schema_type {
        Some(schema_typeset) => match schema_typeset {
            SchemaTypeSet::Single(schema_type) => {
                tokenize_flat_schema(schema_name, &schema, schema_type)
            }
            SchemaTypeSet::Multiple(_items) => todo!(),
        },
        None => todo!(),
    };

    tokenized_schema.to_token_stream()
}

fn tokenize_flat_schema(
    schema_name: String,
    schema: &ObjectSchema,
    schema_type: &SchemaType,
) -> models::TokenizedSchema {
    match schema_type {
        SchemaType::Boolean => todo!(),
        SchemaType::Integer => tokenize_integer_schema(schema_name, schema),
        SchemaType::Number => tokenize_number_schema(schema_name, schema),
        SchemaType::String => tokenize_string_schema(schema_name, schema),
        SchemaType::Array => todo!(),
        SchemaType::Object => todo!(),
        SchemaType::Null => todo!(),
    }
}

fn tokenize_integer_schema(schema_name: String, schema: &ObjectSchema) -> models::TokenizedSchema {
    // if one injects schema_name directly into quote it gets tokenized together with "",
    // which we do not want. Doing it like this drops the ""
    let tokenized_name: TokenStream = schema_name.parse().unwrap();

    let (tokenized_type, imports) = match &schema.format {
        Some(format) => schema_format::formatted_number(format),
        None => schema_format::formatted_number("integer"),
    };

    models::TokenizedSchema {
        tokenized_name,
        tokenized_type,
        imports,
        current_type: models::CurrentType::Type,
    }
}

fn tokenize_number_schema(schema_name: String, schema: &ObjectSchema) -> models::TokenizedSchema {
    // if one injects schema_name directly into quote it gets tokenized together with "",
    // which we do not want. Doing it like this drops the ""
    let tokenized_name: TokenStream = schema_name.parse().unwrap();

    let (tokenized_type, imports) = match &schema.format {
        Some(format) => schema_format::formatted_number(format),
        None => schema_format::default_number(),
    };

    models::TokenizedSchema {
        tokenized_name,
        tokenized_type,
        imports,
        current_type: models::CurrentType::Type,
    }
}

fn tokenize_string_schema(schema_name: String, schema: &ObjectSchema) -> models::TokenizedSchema {
    // if one injects schema_name directly into quote it gets tokenized together with "",
    // which we do not want. Doing it like this drops the ""
    let tokenized_name: TokenStream = schema_name.parse().unwrap();

    let (tokenized_type, imports) = match &schema.format {
        Some(format) => schema_format::formatted_string(format),
        None => schema_format::default_string(),
    };

    models::TokenizedSchema {
        tokenized_name,
        tokenized_type,
        imports,
        current_type: models::CurrentType::Type,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parsing::fixtures;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case("integer", "Age", "type: integer", "type Age = i32 ;")]
    #[case("number", "Height", "type: number", "type Height = f32 ;")]
    #[case("string", "Name", "type: string", "type Name = String ;")]
    fn test_parse_base_cases(
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
}
