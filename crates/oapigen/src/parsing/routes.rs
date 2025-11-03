use crate::models;
use crate::models::schema;
use crate::parsing::errors::ParsingError;
use crate::parsing::typeset::schema_to_rust;
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
                let schema_inputs = models::OperationSchemaInputs {
                    operation_name: &operation_name,
                    response_name: &response_name,
                };

                let resolved_response = response.resolve(spec)?;
                let parsed = respose_to_rust(config, spec, &schema_inputs, &resolved_response)?;
                output.extend(parsed);
            }
        }
    }
    Ok(output)
}

fn respose_to_rust(
    config: &models::Config,
    spec: &oas3::Spec,
    schema_inputs: &models::OperationSchemaInputs,
    response: &spec::Response,
) -> Result<Vec<schema::SchemaAsRust>, ParsingError> {
    let mut v = Vec::with_capacity(response.content.len());
    for (mediatype_name, mediatype) in &response.content {
        let parsed = mediatype_to_rust(config, spec, schema_inputs, mediatype)?;
        match parsed {
            Some(tokenized_schema) => v.push(tokenized_schema),
            None => {}
        }
    }

    Ok(v)
}

fn mediatype_to_rust(
    config: &models::Config,
    spec: &oas3::Spec,
    operation_schema_inputs: &models::OperationSchemaInputs,
    media_type: &spec::MediaType,
) -> Result<Option<schema::SchemaAsRust>, ParsingError> {
    let schema_name = [
        operation_schema_inputs
            .operation_name
            .to_case(Case::UpperCamel),
        "Response".to_string(),
        operation_schema_inputs
            .response_name
            .to_case(Case::UpperCamel),
    ]
    .join("");

    let schema_inputs = models::SchemaInputs {
        schema_name: &schema_name,
    };

    let schema = &media_type.schema;
    if let Some(schema) = schema {
        let parsed_schema = schema.resolve(spec)?;
        let tokens = schema_to_rust(config, &schema_inputs, parsed_schema)?;

        return Ok(Some(tokens));
    }

    Ok(None)
}
