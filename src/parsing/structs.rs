use crate::models;
use anyhow::{Context, Result};

fn parse_structs(spec: oas3::Spec) -> Result<Vec<models::Struct>> {
    let mut structs = Vec::new();

    let components = spec
        .clone()
        .components
        .with_context(|| "failed to extract components from spec")?;

    for (schema_name, schema) in components.schemas {
        let resolved_schema = schema
            .resolve(&spec)
            .with_context(|| format!("failed to resolve schema with name: '{:}'", &schema_name))?;

        let _ = resolved_schema.title.is_none();
    }

    Ok(structs)
}

#[cfg(test)]
mod tests {
    use crate::parsing::structs::parse_structs;

    #[test]
    fn test_parse_structs() {
        let yaml = std::fs::read_to_string("fixtures/one_route_basic_types.yaml").unwrap();
        let spec = oas3::from_yaml(yaml).unwrap();

        let _ = parse_structs(spec);
    }
}
