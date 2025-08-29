use anyhow::{Context, Result};
use oas3::spec::SchemaTypeSet;

use crate::models;

fn parse_structs(spec: oas3::Spec) -> Result<()> {
    let components = spec
        .clone()
        .components
        .with_context(|| "failed to extract components from spec")?;

    for (schema_name, schema) in components.schemas {
        let resolved_schema = schema
            .resolve(&spec)
            .with_context(|| format!("failed to resolve schema with name: '{:}'", &schema_name))?;

        let _ = resolved_schema.title.is_none();

        let a: String;
        match resolved_schema.schema_type {
            Some(t) => match t {
                SchemaTypeSet::Single(i) => a = format!("{:?}", i),
                SchemaTypeSet::Multiple(items) => a = "multiple".to_string(),
            },
            None => todo!(),
        };
        print!("{a}");
        continue;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parsing::structs::parse_structs;

    #[test]
    fn test_parse_structs() {
        let yaml = std::fs::read_to_string("fixtures/one_route_int.yaml").unwrap();
        let spec = oas3::from_yaml(yaml).unwrap();

        let _ = parse_structs(spec);
    }
}
