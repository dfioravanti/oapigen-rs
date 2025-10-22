use crate::models;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

const NAME_IMPORTS_SERDE: &str = "SERDE";

fn get_default_macros() -> (TokenStream, models::Imports) {
    let mut imports = HashMap::new();
    imports.insert(
        NAME_IMPORTS_SERDE.to_string(),
        quote! { use serde::{Deserialize, Serialize}; },
    );

    let default_macros = quote! {
        #[derive(Debug, Deserialize, Serialize)]
    };

    (default_macros, imports)
}

/// Returns the macros that are associated with the current type.
/// By default, we add derive(Debug, Deserialize, Serialize) to allow for reasonable behaviour.
pub(crate) fn get_macros() -> (TokenStream, models::Imports) {
    get_default_macros()
}
