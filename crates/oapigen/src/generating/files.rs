use crate::generating::errors::GeneratingError;
use crate::models::schema;
use std::path::Path;

pub fn write_to_file(
    parsed_schemas: Vec<schema::SchemaAsRust>,
    path: &Path,
) -> Result<(), GeneratingError> {
    Ok(())
}
