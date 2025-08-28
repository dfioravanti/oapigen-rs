use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

// Collects all the imports that were already added to the code.
// It is used to determine if an import should be added or not.
// For example assume that two different structs need
// chrono::DateTime then we add "chrono::DateTime" to the AddedImports
// for the first struct that we hit and we will not add it in the second case since we already have it
type Imports = HashMap<String, TokenStream>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Struct {
    pub struct_name: String,
    pub fields: Vec<Field>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub field_name: String,
    pub field_type: String,
    pub comment: Option<String>,
    pub annotations: Vec<String>,
}

#[derive(Debug)]
pub struct TokenizedStruct {
    pub struct_name: TokenStream,
    pub fields: Vec<TokenizedField>,
}
#[derive(Debug)]
pub struct TokenizedField {
    pub field_name: TokenStream,
    pub field_type: TokenStream,
    pub comment: Option<TokenStream>,
    pub annotations: Vec<TokenStream>,
}

pub struct ParsedSchema {
    tokenized_schema: TokenStream,
    imports: Imports,
}

impl fmt::Display for ParsedSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let imports = self.imports.iter().map(|(k, v)| v).collect::<Vec<_>>();
        let tokenized_schema = &self.tokenized_schema;

        let code = quote! {
            #(
                #imports
            )*

            #tokenized_schema
        }
        .to_string();

        write!(f, "{}", code)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn test_parsed_schema_display() {
        let tokenized_schema = quote! {
            DateTime
        };

        let mut imports = HashMap::new();
        imports.insert(
            "chrono::DateTime".to_string(),
            quote! { use chrono::DateTime },
        );

        let parsed_schema = ParsedSchema {
            tokenized_schema,
            imports,
        };

        let formatted_parsed_schema = format!("{}", parsed_schema);

        assert_yaml_snapshot!(formatted_parsed_schema);
    }
}
