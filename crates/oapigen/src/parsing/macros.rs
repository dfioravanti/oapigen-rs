use crate::models;
use std::collections::{HashMap, HashSet};

const NAME_IMPORTS_SERDE: &str = "SERDE";

fn get_default_macros() -> (HashSet<String>, models::Imports) {
    let mut imports = HashMap::new();
    imports.insert(
        NAME_IMPORTS_SERDE.to_string(),
        "use serde::{Deserialize, Serialize};".to_string(),
    );

    let default_macros = HashSet::from(["#[derive(Debug, Deserialize, Serialize)]".to_string()]);

    (default_macros, imports)
}

/// Returns the macros that are associated with the current type.
/// By default, we add derive(Debug, Deserialize, Serialize) to allow for reasonable behaviour.
pub(crate) fn get_macros() -> (HashSet<String>, models::Imports) {
    get_default_macros()
}
