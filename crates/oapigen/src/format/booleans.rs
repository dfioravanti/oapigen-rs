use crate::models;
use std::collections::HashMap;

/// Formats boolean according to the registry provided in
/// [https://spec.openapis.org/registry/format/].
///
/// The list of supported format is a subset of the registry.
pub fn format_boolean() -> (String, models::Imports) {
    ("bool".to_string(), HashMap::new())
}
