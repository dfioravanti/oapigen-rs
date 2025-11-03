use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

/// Collects all the imports that were already added to the code.
/// It is used to determine if an import should be added or not.
/// For example assume that two different structs need
/// chrono::DateTime then we add "chrono::DateTime" to the Imports
/// for the first struct that we hit, and we will not add it in the second case since we already have it.
pub type Imports = HashMap<String, String>;

#[derive(Debug)]
pub enum CurrentType {
    Type,
    Const,
    Enum,
    Vector,
    Struct,
}

/// SchemaAsRust represents a rust type that was inferred from a OpenAPI specification.
/// It contains all the information needed to render the type correctly.
#[derive(Debug)]
pub struct SchemaAsRust {
    /// The name of the field as a TokenStream
    pub name: String,
    /// The type that the struct uses
    pub rust_type: String,
    /// The macros that will be applied to this type as a TokenStream
    pub macros: HashSet<String>,
    /// The imports needed to make the type compile.
    pub imports: Imports,
    /// The optional comment to the schema
    pub comment: Option<String>,
    /// Is the type optional?
    pub is_optional: bool,

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
            .values()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");

        let output = if imports.is_empty() {
            format!("{}", self.to_token_stream())
        } else {
            format!("{} \n {}", imports, self.to_token_stream())
        };

        let b = match syn::parse_file(&output.to_string()) {
            Ok(b) => b,
            Err(e) => panic!("{}", e),
        };
        let formatted = prettyplease::unparse(&b);

        write!(f, "{}", formatted)
    }
}

impl ToTokens for SchemaAsRust {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let generated_tokens = match self.current_type {
            CurrentType::Type => tokenize_type(self),
            CurrentType::Const => todo!(),
            CurrentType::Enum => todo!(),
            CurrentType::Vector => todo!(),
            CurrentType::Struct => todo!(),
        };

        tokens.clone_from(&generated_tokens);
    }
}

fn tokenize_type(rust_schema: &SchemaAsRust) -> TokenStream {
    let tokenized_name = match &rust_schema.name.parse::<TokenStream>() {
        Ok(v) => v.clone(),
        Err(e) => panic!("{}", format!("cannot turn name to tokens: {}", e)),
    };
    let tokenized_type = match &rust_schema.rust_type.parse::<TokenStream>() {
        Ok(v) => v.clone(),
        Err(e) => panic!("{}", format!("cannot turn rust type to tokens: {}", e)),
    };
    // TODO: rewrite this as something readable, it is just chaining the macros with \n
    let tokenized_macros = match &rust_schema
        .macros
        .clone()
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n")
        .parse::<TokenStream>()
    {
        Ok(v) => v.clone(),
        Err(e) => panic!("{}", format!("cannot turn macros to tokens: {}", e)),
    };
    let tokenized_comment = match &rust_schema.comment {
        None => TokenStream::new(),
        Some(c) => {
            quote! { #[doc = #c] }
        }
    };

    quote! {
        #tokenized_comment
        #tokenized_macros
        struct #tokenized_name(#tokenized_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_parsed_schema_display() {
        let tokenized_schema = "DateTime".to_string();

        let mut imports = HashMap::new();
        imports.insert(
            "chrono::DateTime".to_string(),
            " use chrono::DateTime;".to_string(),
        );

        let parsed_schema = SchemaAsRust {
            name: "time".to_string(),
            rust_type: tokenized_schema,
            macros: HashSet::from(["#[derive(Serialize, Deserialize, Debug)]".to_string()]),
            imports,
            comment: Some("My favourite comment".to_string()),
            is_optional: false,
            current_type: CurrentType::Type,
        };

        let formatted_parsed_schema = format!("{}", parsed_schema);

        assert_snapshot!(formatted_parsed_schema);
    }
}
