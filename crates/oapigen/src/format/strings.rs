//! [https://spec.openapis.org/registry/format/]

use crate::models;
use crate::models::DateTimeLibraries;
use log::warn;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

// the various names for the imports unsed in the various
const NAME_IMPORTS_CHRONO: &str = "chrono_datetime_utc";
const NAME_IMPORTS_JIFF: &str = "jiff_timestamp";

pub(crate) fn default_string() -> (TokenStream, models::Imports) {
    (quote! { String }, HashMap::new())
}

/// Formats strings according to the registry provided in
/// [https://spec.openapis.org/registry/format/].
///
/// The list of supported format is a subset of the registry.
pub fn format_string(config: &models::Config, type_format: &str) -> (TokenStream, models::Imports) {
    let tokenized_type: TokenStream;
    let mut imports = HashMap::new();

    match type_format {
        "date-time" => match config.libraries.datetime {
            DateTimeLibraries::Chrono => {
                tokenized_type = quote! { DateTime<Utc> };
                imports.insert(
                    NAME_IMPORTS_CHRONO.to_string(),
                    quote! { use chrono::{DateTime, Utc}; },
                );
            }
            DateTimeLibraries::Jiff => {
                tokenized_type = quote! { Timestamp };
                imports.insert(
                    NAME_IMPORTS_JIFF.to_string(),
                    quote! { use jiff::Timestamp; },
                );
            }
        },
        _ => {
            warn!("format {type_format} is unknown for strings, defaulting to string");
            return default_string();
        }
    }

    (tokenized_type, imports)
}
