//! [https://spec.openapis.org/registry/format/]

use crate::models;
use log::warn;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

/// Formats numbers (including integers) according to the registry provided in
/// [https://spec.openapis.org/registry/format/].
///
/// The list of supported format is a subset of the registry.
pub fn format_number(type_format: &str) -> (TokenStream, models::Imports) {
    let tokenized_type: TokenStream;
    let imports = HashMap::new();

    match type_format {
        "int64" => tokenized_type = quote! { i64 },
        "integer" | "int32" => tokenized_type = quote! { i32 },
        "int16" => tokenized_type = quote! { i16 },
        "int8" => tokenized_type = quote! { i8 },
        "uint64" => tokenized_type = quote! { u64 },
        "uint32" => tokenized_type = quote! { u32 },
        "uint16" => tokenized_type = quote! { u16 },
        "uint8" => tokenized_type = quote! { u8 },
        "float" | "number " => tokenized_type = quote! { f32 },
        "double" => tokenized_type = quote! { f64 },
        _ => {
            tokenized_type = {
                warn!("format {type_format} is unknown for numbers, defaulting to number");
                quote! { f32 }
            }
        }
    }

    (tokenized_type, imports)
}
