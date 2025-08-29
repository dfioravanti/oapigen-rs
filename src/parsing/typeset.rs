use crate::{models, parsing::schema_format};
use log::warn;
use oas3::spec::{ObjectSchema, Schema, SchemaType, SchemaTypeSet};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub fn tokenize_schema(schema_name: String, schema: ObjectSchema) -> TokenStream {
    let tokenized_schema = match &schema.schema_type {
        Some(schema_typeset) => match schema_typeset {
            SchemaTypeSet::Single(schema_type) => {
                tokenize_flat_schema(schema_name, &schema, schema_type)
            }
            SchemaTypeSet::Multiple(items) => todo!(),
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
    use test_case::test_case;

    use super::*;

    #[test_case("age", "type: integer", "age : i32"; "integer")]
    #[test_case("age", "type: number", "age : f32"; "number")]
    #[test_case("age", "type: string", "age : String"; "string")]
    fn test_parse_base_cases(schema_name: &str, schema_spec: &str, expected: &str) {
        let schema = serde_yaml::from_str::<ObjectSchema>(schema_spec).unwrap();

        let got = tokenize_schema(schema_name.to_string(), schema);

        assert_eq!(got.to_string(), expected.to_string());
    }
}
