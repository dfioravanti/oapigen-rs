use crate::parsing::errors;
use oas3::spec;
use oas3::spec::{SchemaType, SchemaTypeSet};

pub fn parse_schema(schema: spec::ObjectSchema) {
    schema.schema_type;
}

fn validate_schema(schema: spec::ObjectSchema) -> Result<(), errors::ParsingError> {
    Ok(())
}

fn is_type(typeset: SchemaTypeSet, schema_type: SchemaType) -> bool {
    match typeset {
        SchemaTypeSet::Single(t) => t.eq(&schema_type),
        SchemaTypeSet::Multiple(items) => items.iter().any(|t| t.eq(&schema_type)),
    }
}
