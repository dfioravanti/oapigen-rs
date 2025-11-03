mod booleans;
pub(crate) mod nullable;
pub(crate) mod numbers;
pub(crate) mod strings;

pub use booleans::format_boolean;
pub use nullable::format_null;
pub use numbers::format_number;
pub use strings::format_string;

pub const DEFAULT_NUMBER: &str = "float";
pub const DEFAULT_INTEGER: &str = "integer";
pub const DEFAULT_STRING: &str = "string";
