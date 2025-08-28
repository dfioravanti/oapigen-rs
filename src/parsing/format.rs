//! [https://spec.openapis.org/registry/format/]

use log::warn;
use proc_macro2::TokenStream;
use quote::quote;

pub fn type_for_format_string(type_format: &str) -> TokenStream {
    match type_format {
        "time" => {
            todo!()
        }
        _ => {
            warn!("format {type_format} is unknown for strings, defaulting to string");
            quote! { String }
        }
    }
}
