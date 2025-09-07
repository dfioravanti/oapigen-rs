#[derive(Debug, serde::Serialize)]
pub struct Struct {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, serde::Serialize)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub rust_type: String,
    pub is_required: bool,
}

pub struct ConstantField {
    pub name: String,
    pub description: Option<String>,
    pub rust_type: String,
    pub value: serde_json::Value,
}
