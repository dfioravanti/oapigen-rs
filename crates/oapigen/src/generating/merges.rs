use crate::models;
use crate::models::schema;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashSet;

pub fn merge_schemas(schemas: Vec<schema::SchemaAsRust>) -> TokenStream {
    let imports: Vec<_> = schemas.iter().map(|schema| &schema.imports).collect();

    let unique_types: HashSet<_> = HashSet::from_iter(schemas.iter());
    let types = unique_types.iter().fold(TokenStream::new(), |mut acc, s| {
        acc.extend(s.to_token_stream());
        acc
    });

    let merged_imports = merge_imports(imports);
    let mut output = match merged_imports.parse::<TokenStream>() {
        Ok(tokens) => tokens,
        Err(e) => {
            panic!("{}", format!("cannot turn imports to tokens: {}", e))
        }
    };
    output.extend(types);
    output
}

/// merge_imports merges the [TokenStream] that represent the imports used by the models.
/// So for examples if we have two models that use `use chrono::DateTime;` the resulting
/// [TokenStream] will contain only one mention of `use chrono::DateTime;`.
fn merge_imports(all_imports: Vec<&models::Imports>) -> String {
    let mut seen: HashSet<String> = HashSet::new();
    let mut output = "".to_string();
    for imports in all_imports {
        for (import_as_string, actual_import) in imports {
            if !seen.contains(import_as_string) {
                seen.insert(import_as_string.clone());
                output = vec![output, actual_import.clone()].join("\n");
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CurrentType;
    use quote::quote;

    #[test]
    fn test_merge_imports() {
        let mut imports_1 = models::Imports::new();
        imports_1.insert(
            "chrono::DateTime".to_string(),
            "use chrono::DateTime;".to_string(),
        );

        let mut imports_2 = models::Imports::new();
        imports_2.insert(
            "chrono::DateTime".to_string(),
            "use chrono::DateTime;".to_string(),
        );
        imports_2.insert(
            "crate::generating::errors::GeneratingError".to_string(),
            "use crate::generating::errors::GeneratingError;".to_string(),
        );

        let imports = vec![&imports_1, &imports_2];

        let got = merge_imports(imports);
        insta::assert_snapshot!(got.to_string());
    }

    #[test]
    fn test_merge_schema() {
        let mut imports_1 = models::Imports::new();
        imports_1.insert("chrono".to_string(), "use chrono;".to_string());
        let schema1 = schema::SchemaAsRust {
            name: "user_time".to_string(),
            rust_type: "chrono::DateTime<chrono::Utc>".to_string(),
            macros: HashSet::from(["#[derive(Serialize, Deserialize, Debug)]".to_string()]),
            comment: None,
            is_optional: false,
            imports: imports_1,
            current_type: CurrentType::Type,
        };

        let schemas = vec![schema1];
        let got = merge_schemas(schemas);

        insta::assert_snapshot!(got.to_string());
    }
}
