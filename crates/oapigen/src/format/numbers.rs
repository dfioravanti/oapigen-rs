//! [https://spec.openapis.org/registry/format/]

use crate::models;
use log::warn;
use std::collections::HashMap;

/// Formats numbers (including integers) according to the registry provided in
/// [https://spec.openapis.org/registry/format/].
///
/// The list of supported format is a subset of the registry.
pub fn format_number(type_format: &str) -> (String, models::Imports) {
    let rust_type = match type_format {
        "int64" => "i64",
        "integer" | "int32" => "i32",
        "int16" => "i16",
        "int8" => "i8",
        "uint64" => "u64",
        "uint32" => "u32",
        "uint16" => "u16",
        "uint8" => "u8",
        "float" | "number " => "f32",
        "double" => "f64",
        _ => {
            warn!("format {type_format} is unknown for numbers, defaulting to number");
            "f32"
        }
    };

    (rust_type.to_string(), HashMap::new())
}
