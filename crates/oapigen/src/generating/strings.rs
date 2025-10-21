use crate::generating::errors::GeneratingError;
use crate::generating::merges::merge_schemas;
use crate::models;
use crate::parsing::specs::parse_specs;

pub fn spec_to_rust(config: &models::Config, spec: oas3::Spec) -> Result<String, GeneratingError> {
    let parsed_schemas = parse_specs(config, spec)?;
    let got = merge_schemas(parsed_schemas);
    let b = syn::parse_file(&got.to_string())?;
    let formatted = prettyplease::unparse(&b);
    Ok(formatted)
}
