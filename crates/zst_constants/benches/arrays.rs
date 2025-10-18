use brunch::Bench;
use serde::{Deserialize, Serialize};
use zst_constants::ZSTConstant;

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "null")]
struct MyZSTNull;
#[derive(Serialize, Deserialize, Debug)]
struct NullStruct {
    data: MyZSTNull,
    more_data: String,
}

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "string", const_value = "v32")]
struct MyZSTString;
#[derive(Serialize, Deserialize, Debug)]
struct StringStruct {
    data: MyZSTString,
    more_data: String,
}

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "integer", const_value = "32")]
struct MyZSTInteger;
#[derive(Serialize, Deserialize, Debug)]
struct IntegerStruct {
    data: MyZSTInteger,
    more_data: String,
}

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "number", const_value = "32.78")]
struct MyZSTNumber;
#[derive(Serialize, Deserialize, Debug)]
struct NumberStruct {
    data: MyZSTNumber,
    more_data: String,
}

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "array", const_value = "[1,2,3]")]
struct MyZSTArray;
#[derive(Serialize, Deserialize, Debug)]
struct ArrayStruct {
    data: MyZSTArray,
    more_data: String,
}

brunch::benches!(
    Bench::new("null - deserializing")
        .with_samples(100_000)
        .run(|| {
            let my_object = NullStruct {
                data: MyZSTNull,
                more_data: "some important data".to_string(),
            };

            let _ = serde_json::to_string_pretty(&my_object).unwrap();
        }),
    Bench::new("null - serialising")
        .with_samples(100_000)
        .run(|| {
            let valid_json_input = r#"{ "data": null, "more_data": "payload" }"#;
            let _: NullStruct = serde_json::from_str(valid_json_input).unwrap();
        }),
    Bench::new("string - deserializing")
        .with_samples(100_000)
        .run(|| {
            let my_object = StringStruct {
                data: MyZSTString,
                more_data: "some important data".to_string(),
            };

            let _ = serde_json::to_string_pretty(&my_object).unwrap();
        }),
    Bench::new("string - serialising")
        .with_samples(100_000)
        .run(|| {
            let valid_json_input = r#"{ "data": "v32", "more_data": "payload" }"#;
            let _: StringStruct = serde_json::from_str(valid_json_input).unwrap();
        }),
    Bench::new("integer - deserializing")
        .with_samples(100_000)
        .run(|| {
            let my_object = IntegerStruct {
                data: MyZSTInteger,
                more_data: "some important data".to_string(),
            };

            let _ = serde_json::to_string_pretty(&my_object).unwrap();
        }),
    Bench::new("integer - serialising")
        .with_samples(100_000)
        .run(|| {
            let valid_json_input = r#"{ "data": 32, "more_data": "payload" }"#;
            let _: IntegerStruct = serde_json::from_str(valid_json_input).unwrap();
        }),
    Bench::new("number - deserializing")
        .with_samples(100_000)
        .run(|| {
            let my_object = NumberStruct {
                data: MyZSTNumber,
                more_data: "some important data".to_string(),
            };

            let _ = serde_json::to_string_pretty(&my_object).unwrap();
        }),
    Bench::new("number - serialising")
        .with_samples(100_000)
        .run(|| {
            let valid_json_input = r#"{ "data": 32.78, "more_data": "payload" }"#;
            let _: NumberStruct = serde_json::from_str(valid_json_input).unwrap();
        }),
    Bench::new("array - deserializing")
        .with_samples(100_000)
        .run(|| {
            let my_object = ArrayStruct {
                data: MyZSTArray,
                more_data: "some important data".to_string(),
            };

            let _ = serde_json::to_string_pretty(&my_object).unwrap();
        }),
    Bench::new("array - serialising")
        .with_samples(100_000)
        .run(|| {
            let valid_json_input = r#"{ "data": [1,2,3], "more_data": "payload" }"#;
            let _: ArrayStruct = serde_json::from_str(valid_json_input).unwrap();
        })
);
