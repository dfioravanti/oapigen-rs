//! [https://spec.openapis.org/registry/format/]

use std::collections::HashMap;

use crate::models;
use log::warn;
use proc_macro2::TokenStream;
use quote::quote;

pub fn default_string() -> (TokenStream, models::Imports) {
    (quote! { String }, HashMap::new())
}

/// Formats strings according to the registry provided in
/// [https://spec.openapis.org/registry/format/].
///
/// The list of supported formats is a subset of the registry.
pub fn formatted_string(type_format: &str) -> (TokenStream, models::Imports) {
    let tokenized_type: TokenStream;
    let mut imports = HashMap::new();

    match type_format {
        "time" => {
            tokenized_type = quote! { DateTime };
            imports.insert(
                "chrono::DateTime".to_string(),
                quote! { use chrono::DateTime },
            );
        }
        _ => {
            warn!("format {type_format} is unknown for strings, defaulting to string");
            return default_string();
        }
    }

    (tokenized_type, imports)
}

pub fn default_number() -> (TokenStream, models::Imports) {
    (quote! { f32 }, HashMap::new())
}

/// Formats numbers (including integers) according to the registry provided in
/// [https://spec.openapis.org/registry/format/].
///
/// The list of supported formats is a subset of the registry.
pub(crate) fn formatted_number(type_format: &str) -> (TokenStream, models::Imports) {
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
        "double" => tokenized_type = quote! { f64 },
        _ => {
            warn!("format {type_format} is unknown for numbers, defaulting to number");
            return default_number();
        }
    }

    (tokenized_type, imports)
}
