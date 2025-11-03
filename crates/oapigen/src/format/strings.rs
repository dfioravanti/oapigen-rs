//! [https://spec.openapis.org/registry/format/]

use crate::models;
use crate::models::DateTimeLibraries;
use log::warn;
use std::collections::HashMap;

// the various names for the imports unsed in the various
const NAME_IMPORTS_CHRONO: &str = "chrono_datetime_utc";
const NAME_IMPORTS_JIFF: &str = "jiff_timestamp";

/// Formats strings according to the registry provided in
/// [https://spec.openapis.org/registry/format/].
///
/// The list of supported format is a subset of the registry.
pub fn format_string(config: &models::Config, type_format: &str) -> (String, models::Imports) {
    let mut imports = HashMap::new();

    let rust_type = match type_format {
        "date-time" => match config.libraries.datetime {
            DateTimeLibraries::Chrono => {
                imports.insert(
                    NAME_IMPORTS_CHRONO.to_string(),
                    "use chrono::{DateTime, Utc};".to_string(),
                );
                "DateTime<Utc>"
            }
            DateTimeLibraries::Jiff => {
                imports.insert(
                    NAME_IMPORTS_JIFF.to_string(),
                    "use jiff::Timestamp;".to_string(),
                );
                "Timestamp"
            }
        },
        _ => {
            warn!("format {type_format} is unknown for strings, defaulting to string");
            "String"
        }
    };

    (rust_type.to_string(), imports)
}
