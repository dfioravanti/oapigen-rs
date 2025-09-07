use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::{collections::HashMap, fmt};

/// Collects all the imports that were already added to the code.
/// It is used to determine if an import should be added or not.
/// For example assume that two different structs need
/// chrono::DateTime then we add "chrono::DateTime" to the Imports
/// for the first struct that we hit and we will not add it in the second case since we already have it.
pub type Imports = HashMap<String, TokenStream>;

#[derive(Debug)]
pub enum CurrentType {
    Type,
    Const,
    Enum,
    Vec,
    Struct,
}

/// TokenizedType represents a rust type that was inferred from a OpenAPI specification.
/// It contains all the information needed to render the type correctly.
#[derive(Debug)]
pub struct TokenizedSchema {
    /// The name of the field as a TokenStream
    pub tokenized_name: TokenStream,
    /// The type represented as a TokenStream
    pub tokenized_type: TokenStream,
    /// The imports needed to make the type compile.
    pub imports: Imports,

    pub current_type: CurrentType,
}

pub struct TokenizedValue {
    pub tokenized_name: Option<TokenStream>,
    pub tokenized_value: Option<TokenStream>,
}

impl fmt::Display for TokenizedSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let imports = self
            .imports
            .iter()
            .map(|(_, v)| v.to_string())
            .collect::<Vec<_>>();

        write!(
            f,
            "imports => {:?} \n schema => {:?}",
            imports,
            self.to_token_stream().to_string()
        )
    }
}

impl quote::ToTokens for TokenizedSchema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let generated_tokens = match self.current_type {
            CurrentType::Type => tokenize_type(self),
            CurrentType::Const => todo!(),
            CurrentType::Enum => todo!(),
            CurrentType::Vec => todo!(),
            CurrentType::Struct => todo!(),
        };

        tokens.clone_from(&generated_tokens);
    }
}

fn tokenize_type(tokenized_schema: &TokenizedSchema) -> TokenStream {
    let tokenized_name = &tokenized_schema.tokenized_name;
    let tokenized_type = &tokenized_schema.tokenized_type;

    quote! {

        type #tokenized_name = #tokenized_type;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

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

        let parsed_schema = TokenizedSchema {
            tokenized_name: quote! { time },
            tokenized_type: tokenized_schema,
            imports,
            current_type: CurrentType::Type,
        };

        let formatted_parsed_schema = format!("{}", parsed_schema);

        assert_yaml_snapshot!(formatted_parsed_schema);
    }
}
