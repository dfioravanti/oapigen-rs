use crate::models;
use crate::models::schema;
use crate::parsing::errors::ParsingError;
use crate::parsing::typeset::tokenize_schema;
use convert_case::{Case, Casing};
use oas3::spec;
use std::option::Option;

pub(crate) fn parse_routes(
    config: &models::Config,
    spec: &oas3::Spec,
) -> Result<Vec<schema::SchemaAsRust>, ParsingError> {
    let mut output = Vec::new();

    let paths = &spec.paths;

    if let Some(paths) = paths {
        for (route_name, path) in paths {
            if path.get.is_some() {
                let parsed =
                    parse_operation(config, spec, &"Get".to_string(), route_name, &path.get)?;
                output.extend(parsed);
            }
        }
    }

    Ok(output)
}

fn parse_operation(
    config: &models::Config,
    spec: &oas3::Spec,
    method_name: &String,
    route_name: &String,
    operation: &Option<spec::Operation>,
) -> Result<Vec<schema::SchemaAsRust>, ParsingError> {
    let mut output = Vec::new();
    if let Some(operation) = operation {
        let operation_name = match &operation.operation_id {
            Some(operation_id) => operation_id.to_case(Case::UpperCamel),
            None => [method_name, &route_name.to_case(Case::UpperCamel)]
                .map(String::as_str)
                .concat(),
        };

        let responses = &operation.responses;
        if let Some(responses) = responses {
            for (response_name, response) in responses {
                let resolved_response = response.resolve(spec)?;
                let parsed = parse_response(
                    config,
                    spec,
                    &operation_name,
                    &response_name,
                    &resolved_response,
                )?;
                output.extend(parsed);
            }
        }
    }
    Ok(output)
}

fn parse_response(
    config: &models::Config,
    spec: &oas3::Spec,
    operation_name: &String,
    response_name: &String,
    response: &spec::Response,
) -> Result<Vec<schema::SchemaAsRust>, ParsingError> {
    let mut v = Vec::with_capacity(response.content.len());
    for (mediatype_name, mediatype) in &response.content {
        let parsed = parse_mediatype(config, spec, operation_name, response_name, mediatype)?;
        match parsed {
            Some(tokenized_schema) => v.push(tokenized_schema),
            None => {}
        }
    }

    Ok(v)
}

fn parse_mediatype(
    config: &models::Config,
    spec: &oas3::Spec,
    operation_name: &String,
    response_name: &String,
    media_type: &spec::MediaType,
) -> Result<Option<schema::SchemaAsRust>, ParsingError> {
    let schema_name = [
        operation_name.to_case(Case::UpperCamel),
        "Response".to_string(),
        response_name.to_case(Case::UpperCamel),
    ]
    .join("");

    let schema = &media_type.schema;
    if let Some(schema) = schema {
        let parsed_schema = schema.resolve(spec)?;
        let tokens = tokenize_schema(config, schema_name, parsed_schema)?;

        return Ok(Some(tokens));
    }

    Ok(None)
}
