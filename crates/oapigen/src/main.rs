mod models;
mod parsing;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::json;

#[derive(Debug)]
struct ApiVersionV2;

impl Serialize for ApiVersionV2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("v2")
    }
}

impl<'de> Deserialize<'de> for ApiVersionV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let incoming = String::deserialize(deserializer)?;

        // The core validation logic
        if incoming == "v2" {
            Ok(ApiVersionV2)
        } else {
            Err(de::Error::custom(format!(
                "expected string \"v2\", found \"{}\"",
                incoming
            )))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MyApiObject {
    // This field is guaranteed to be "v2" if deserialization succeeds.
    // It adds no size to the struct at runtime.
    pub api_version: ApiVersionV2,
    pub data: String,
}

fn main() {
    // --- Serialization ---
    let my_object = MyApiObject {
        api_version: ApiVersionV2, // The only possible value
        data: "some important data".to_string(),
    };

    let json_output = serde_json::to_string_pretty(&my_object).unwrap();
    println!("Serialized JSON:\n{}", json_output);
    // Output will be:
    // {
    //   "api_version": "v2",
    //   "data": "some important data"
    // }

    // --- Deserialization (Success) ---
    let valid_json_input = r#"{ "api_version": "v2", "data": "payload" }"#;
    let deserialized_ok: MyApiObject = serde_json::from_str(valid_json_input).unwrap();
    println!("\nSuccessfully deserialized: {:?}", deserialized_ok);

    // --- Deserialization (Failure) ---
    let invalid_json_input = r#"{ "api_version": "v1", "data": "old payload" }"#;
    let deserialized_err = serde_json::from_str::<MyApiObject>(invalid_json_input);
    println!("\nFailed to deserialize: {:?}", deserialized_err);
    // Output will be:
    // Err(Error("expected string \"v2\", found \"v1\"", line: 1, column: 22))
}
