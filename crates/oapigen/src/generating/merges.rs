use crate::models;
use crate::models::schema;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashSet;

pub fn merge_schemas(schemas: Vec<schema::SchemaAsRust>) -> TokenStream {
    let imports: Vec<_> = schemas.iter().map(|schema| &schema.imports).collect();
    let mut output = merge_imports(imports);

    let unique_types: HashSet<_> = HashSet::from_iter(schemas.iter());
    let types = unique_types.iter().fold(TokenStream::new(), |mut acc, s| {
        acc.extend(s.to_token_stream());
        acc
    });

    output.extend(types);
    output
}

/// merge_imports merges the [TokenStream] that represent the imports used by the models.
/// So for examples if we have two models that use `use chrono::DateTime;` the resulting
/// [TokenStream] will contain only one mention of `use chrono::DateTime;`.
fn merge_imports(all_imports: Vec<&models::Imports>) -> TokenStream {
    let mut seen: HashSet<String> = HashSet::new();
    let mut output = TokenStream::new();
    for imports in all_imports {
        for (import_as_string, actual_import) in imports {
            if !seen.contains(import_as_string) {
                seen.insert(import_as_string.clone());
                output.extend(actual_import.clone().into_iter());
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
            quote! { use chrono::DateTime; },
        );

        let mut imports_2 = models::Imports::new();
        imports_2.insert(
            "chrono::DateTime".to_string(),
            quote! { use chrono::DateTime; },
        );
        imports_2.insert(
            "crate::generating::errors::GeneratingError".to_string(),
            quote! { use crate::generating::errors::GeneratingError; },
        );

        let imports = vec![&imports_1, &imports_2];
        let want = quote! {
            use chrono::DateTime;
            use crate::generating::errors::GeneratingError;
        };

        let got = merge_imports(imports);
        assert_eq!(want.to_string(), got.to_string());
    }

    #[test]
    fn test_merge_schema() {
        let mut imports_1 = models::Imports::new();
        imports_1.insert("chrono".to_string(), quote! { use chrono; });
        let schema1 = schema::SchemaAsRust {
            tokenized_name: quote! { user_time },
            tokenized_type: quote! { chrono::DateTime<chrono::Utc> },
            imports: imports_1,
            current_type: CurrentType::Type,
        };

        let schemas = vec![schema1];
        let got = merge_schemas(schemas);
        let want = quote! {
            use chrono;

            type user_time = chrono::DateTime<chrono::Utc>;
        };
        assert_eq!(want.to_string(), got.to_string());
    }
}
