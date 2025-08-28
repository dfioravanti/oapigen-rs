use crate::parsing::format::type_for_format_string;
use log::warn;
use oas3::spec::{ObjectSchema, Schema, SchemaType, SchemaTypeSet};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub fn tokenize_schema2(schema_name: String, schema: ObjectSchema) -> TokenStream {
    match &schema.schema_type {
        Some(schema_typeset) => match schema_typeset {
            SchemaTypeSet::Single(schema_type) => {
                tokenize_schema_type(schema_name, &schema, schema_type)
            }
            SchemaTypeSet::Multiple(items) => todo!(),
        },
        None => todo!(),
    }
}

fn tokenize_schema_type(
    schema_name: String,
    schema: &ObjectSchema,
    schema_type: &SchemaType,
) -> TokenStream {
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

/// [https://spec.openapis.org/registry/format/]
fn tokenize_integer_schema(schema_name: String, schema: &ObjectSchema) -> TokenStream {
    // if one injects schema_name directly into quote it gets tokenized together with "",
    // which we do not want. Doing it like this drops the ""
    let tokenized_name: TokenStream = schema_name.parse().unwrap();

    let tokenized_type = match &schema.format {
        Some(format) => todo!(),
        None => quote! { i64 },
    };

    quote! {
        #tokenized_name : #tokenized_type
    }
}

fn tokenize_number_schema(schema_name: String, schema: &ObjectSchema) -> TokenStream {
    // if one injects schema_name directly into quote it gets tokenized together with "",
    // which we do not want. Doing it like this drops the ""
    let tokenized_name: TokenStream = schema_name.parse().unwrap();

    let tokenized_type = match &schema.format {
        Some(format) => todo!(),
        None => quote! { float },
    };

    quote! {
        #tokenized_name : #tokenized_type
    }
}

fn tokenize_string_schema(schema_name: String, schema: &ObjectSchema) -> TokenStream {
    // if one injects schema_name directly into quote it gets tokenized together with "",
    // which we do not want. Doing it like this drops the ""
    let tokenized_name: TokenStream = schema_name.parse().unwrap();

    let tokenized_type = match &schema.format {
        Some(format) => type_for_format_string(format),
        None => quote! { String },
    };

    quote! {
        #tokenized_name : #tokenized_type
    }
}

// This module deals with parsing of the various formats supported by this library.
// The list of supported formats is a subset of
// [https://spec.openapis.org/registry/format/]
pub fn parse_format(schema_type: SchemaType, format: String) -> TokenStream {
    match format {
        _ => todo!(), // _ => {
                      //     warn!(
                      //         "unknown format {:} for type {:?}, using the default for {:?}",
                      //         format, schema_type, schema_type
                      //     );
                      // }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("age", "type: integer", "age : i64"; "integer")]
    #[test_case("age", "type: number", "age : float"; "number")]
    #[test_case("age", "type: string", "age : String"; "string")]
    fn test_parse_base_cases(schema_name: &str, schema_spec: &str, expected: &str) {
        let schema = serde_yaml::from_str::<ObjectSchema>(schema_spec).unwrap();

        let got = tokenize_schema2(schema_name.to_string(), schema);

        assert_eq!(got.to_string(), expected.to_string());
    }
}
