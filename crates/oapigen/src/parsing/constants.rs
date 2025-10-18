use oas3::spec::ObjectSchema;
use serde_json::Value;

use crate::models;

fn parse_constants(schema_name: String, schema: ObjectSchema) -> models::SchemaAsRust {
    match schema.const_value.unwrap() {
        Value::Null => todo!(),
        Value::Bool(v) => todo!(),
        Value::Number(v) => todo!(),
        Value::String(v) => todo!(),
        Value::Array(values) => todo!(),
        Value::Object(field_to_value) => todo!(),
    }
}

fn parse_value(original_value: Value) {
    let mut stack = Vec::new();

    stack.push(original_value);

    while let Some(value) = stack.pop() {}
}
