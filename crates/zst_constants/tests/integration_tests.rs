use insta::assert_json_snapshot;
use rstest::*;
use serde::{Deserialize, Serialize};
use zst_constants::ZSTConstant;

// as none struct

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "none")]
struct MyZSTNone;

#[derive(Serialize, Deserialize, Debug)]
struct NoneStruct {
    none_value: MyZSTNone,
    data: String,
}

#[rstest]
fn test_none_constant_zst_serializes_correctly() {
    let my_object = NoneStruct {
        none_value: MyZSTNone,
        data: "some important data".to_string(),
    };

    let json_output = serde_json::to_string_pretty(&my_object).unwrap();
    assert_json_snapshot!(json_output);
}

#[rstest]
fn test_none_constant_zst_deserializes_correctly() {
    let valid_json_input = r#"{ "none_value": null, "data": "payload" }"#;
    let _: NoneStruct = serde_json::from_str(valid_json_input).unwrap();
}

// as bool struct

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "bool", const_value = "true")]
struct MyZSTBool;

#[derive(Serialize, Deserialize, Debug)]
struct BoolStruct {
    data: MyZSTBool,
    more_data: String,
}

#[rstest]
fn test_bool_constant_zst_serializes_correctly() {
    let my_object = BoolStruct {
        data: MyZSTBool,
        more_data: "some important data".to_string(),
    };

    let json_output = serde_json::to_string_pretty(&my_object).unwrap();
    assert_json_snapshot!(json_output);
}

#[rstest]
fn test_bool_constant_zst_deserializes_correctly() {
    let valid_json_input = r#"{ "data": true, "more_data": "payload" }"#;
    let _: BoolStruct = serde_json::from_str(valid_json_input).unwrap();
}

// as string structs

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "string", const_value = "v2")]
struct MyZSTString;

#[derive(Serialize, Deserialize, Debug)]
struct StringStruct {
    api_version: MyZSTString,
    data: String,
}

#[rstest]
fn test_as_constant_with_string() {
    let my_object = StringStruct {
        api_version: MyZSTString,
        data: "some important data".to_string(),
    };

    let json_output = serde_json::to_string_pretty(&my_object).unwrap();
    assert_json_snapshot!(json_output);

    let valid_json_input = r#"{ "api_version": "v2", "data": "payload" }"#;
    let _: StringStruct = serde_json::from_str(valid_json_input).unwrap();
}

// as struct structs

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(
    const_type = "struct",
    const_value = r#"{ "type": "cash", "currency": "EUR" }"#
)]
struct CashPaymentMethod;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    payment_method: CashPaymentMethod,
    amount: f64,
}

#[rstest]
fn test_as_constant_with_struct_serialize_correctly() {
    let transaction = Transaction {
        payment_method: CashPaymentMethod,
        amount: 42.50,
    };

    let json_output = serde_json::to_string_pretty(&transaction).unwrap();
    assert_json_snapshot!(json_output);
}

#[rstest]
fn test_as_constant_with_struct_deserialize_correctly() {
    let valid_json = r#"{
        "payment_method": { "type": "cash", "currency": "EUR" },
        "amount": 100.0
    }"#;
    let _: Transaction = serde_json::from_str(valid_json).unwrap();
}

#[rstest]
#[should_panic]
fn test_as_constant_with_struct_does_not_deserialize_different_values() {
    let valid_json = r#"{
        "payment_method": { "type": "cash", "currency": "USD" },
        "amount": 100.0
    }"#;
    let _: Transaction = serde_json::from_str(valid_json).unwrap();
}

// as integer struct

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "integer", const_value = "2")]
struct MyZSTInteger;

#[derive(Serialize, Deserialize, Debug)]
struct IntegerStruct {
    api_version: MyZSTInteger,
    data: String,
}

#[rstest]
fn test_integer_constant_zst_serializes_correctly() {
    let my_object = IntegerStruct {
        api_version: MyZSTInteger,
        data: "some important data".to_string(),
    };

    let json_output = serde_json::to_string_pretty(&my_object).unwrap();
    assert_json_snapshot!(json_output);
}

#[rstest]
fn test_integer_constant_zst_deserializes_correctly() {
    let valid_json_input = r#"{ "api_version": 2, "data": "payload" }"#;
    let _: IntegerStruct = serde_json::from_str(valid_json_input).unwrap();
}

// as number struct

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "number", const_value = "2.3")]
struct MyZSTNumber;

#[derive(Serialize, Deserialize, Debug)]
struct NumberStruct {
    api_version: MyZSTNumber,
    data: String,
}

#[rstest]
fn test_number_constant_zst_serializes_correctly() {
    let my_object = NumberStruct {
        api_version: MyZSTNumber,
        data: "some important data".to_string(),
    };

    let json_output = serde_json::to_string_pretty(&my_object).unwrap();
    assert_json_snapshot!(json_output);
}

#[rstest]
fn test_number_constant_zst_deserializes_correctly() {
    let valid_json_input = r#"{ "api_version": 2.3, "data": "payload" }"#;
    let _: NumberStruct = serde_json::from_str(valid_json_input).unwrap();
}

// as number struct

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "array", const_value = "[1,2,3]")]
struct MyZSTArray;

#[derive(Serialize, Deserialize, Debug)]
struct ArrayStruct {
    data: MyZSTArray,
    more_data: String,
}

#[rstest]
fn test_array_constant_zst_serializes_correctly() {
    let my_object = ArrayStruct {
        data: MyZSTArray,
        more_data: "some important data".to_string(),
    };

    let json_output = serde_json::to_string_pretty(&my_object).unwrap();
    assert_json_snapshot!(json_output);
}

#[rstest]
fn test_array_constant_zst_deserializes_correctly() {
    let valid_json_input = r#"{ "data": [1,2,3], "more_data": "payload" }"#;
    let _: ArrayStruct = serde_json::from_str(valid_json_input).unwrap();
}
