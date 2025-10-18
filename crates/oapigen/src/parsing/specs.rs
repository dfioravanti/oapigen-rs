use crate::models::schema;
use crate::parsing::errors::ParsingError;
use crate::parsing::routes::parse_routes;
use oas3::spec;

pub fn parse_specs(spec: oas3::Spec) -> Result<Vec<schema::SchemaAsRust>, ParsingError> {
    let mut parsed_schemas = Vec::new();

    let components = spec.clone().components;
    if let Some(components) = components {
        let parsed_schemas_from_routes = parse_routes(&spec)?;
        parsed_schemas.extend(parsed_schemas_from_routes);

        for (schema_name, schema) in components.schemas {
            let resolved_schema = schema.resolve(&spec)?;

            let _ = resolved_schema.title.is_none();

            match resolved_schema.schema_type {
                Some(t) => match t {
                    spec::SchemaTypeSet::Single(i) => todo!(),
                    spec::SchemaTypeSet::Multiple(_) => todo!(),
                },
                None => todo!(),
            };
        }

        return Ok(parsed_schemas);
    }

    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use crate::parsing::specs::parse_specs;
    use rstest::rstest;

    #[rstest]
    #[case("one route int", "fixtures/one_route_int.yaml")]
    fn test_parse_structs(#[case] name: &str, #[case] path: &str) {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(name);

        let yaml = std::fs::read_to_string(path).unwrap();
        let spec = oas3::from_yaml(yaml).unwrap();

        let got = parse_specs(spec).unwrap();
        let got_as_strings = got.iter().map(|v| v.to_string()).collect::<Vec<_>>();
        settings.bind(|| {
            insta::assert_yaml_snapshot!(got_as_strings);
        });
    }
}
