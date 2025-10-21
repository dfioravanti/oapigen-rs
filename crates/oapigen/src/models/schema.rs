use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::hash::Hash;
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

/// SchemaAsRust represents a rust type that was inferred from a OpenAPI specification.
/// It contains all the information needed to render the type correctly.
#[derive(Debug)]
pub struct SchemaAsRust {
    /// The name of the field as a TokenStream
    pub tokenized_name: TokenStream,
    /// The type represented as a TokenStream
    pub tokenized_type: TokenStream,
    /// The imports needed to make the type compile.
    pub imports: Imports,

    pub current_type: CurrentType,
}
impl Eq for SchemaAsRust {}
impl PartialEq for SchemaAsRust {
    fn eq(&self, other: &Self) -> bool {
        self.to_token_stream().to_string() == other.to_token_stream().to_string()
    }
}

impl Hash for SchemaAsRust {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_token_stream().to_string().hash(state);
    }
}

impl fmt::Display for SchemaAsRust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let imports = self
            .imports
            .iter()
            .fold(TokenStream::new(), |mut acc, (_, v)| {
                acc.extend(v.clone());
                acc
            });

        if imports.is_empty() {
            write!(f, "{}", self.to_token_stream())
        } else {
            write!(f, "{} \n {}", imports, self.to_token_stream())
        }
    }
}

impl ToTokens for SchemaAsRust {
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

fn tokenize_type(tokenized_schema: &SchemaAsRust) -> TokenStream {
    let tokenized_name = &tokenized_schema.tokenized_name;
    let tokenized_type = &tokenized_schema.tokenized_type;

    quote! { struct #tokenized_name(#tokenized_type); }
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
            quote! { use chrono::DateTime; },
        );

        let parsed_schema = SchemaAsRust {
            tokenized_name: quote! { time },
            tokenized_type: tokenized_schema,
            imports,
            current_type: CurrentType::Type,
        };

        let formatted_parsed_schema = format!("{}", parsed_schema);

        assert_yaml_snapshot!(formatted_parsed_schema);
    }
}
