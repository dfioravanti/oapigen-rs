use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Struct {
    pub struct_name: String,
    pub fields: Vec<Field>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub field_name: String,
    pub field_type: String,
    pub comment: Option<String>,
    pub annotations: Vec<String>,
}
