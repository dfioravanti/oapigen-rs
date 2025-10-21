pub(crate) mod numbers;
pub(crate) mod strings;
pub use numbers::format_number;
pub use strings::format_string;

pub const DEFAULT_NUMBER: &str = "float";
pub const DEFAULT_INTEGER: &str = "integer";
pub const DEFAULT_STRING: &str = "string";
