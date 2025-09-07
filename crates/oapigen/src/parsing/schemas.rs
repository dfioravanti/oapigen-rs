use crate::parsing::errors;
use oas3::spec;

fn parse_schema(schema: spec::ObjectSchema) {
    schema;
}

fn validate_schema(schema: spec::ObjectSchema) -> Result<(), errors::ParsingError> {
    Ok(())
}
