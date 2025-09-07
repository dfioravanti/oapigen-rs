use insta::assert_json_snapshot;
use rstest::*;
use serde::{Deserialize, Serialize};
use zst_constants::ZSTConstant;

// as string structs

#[derive(Debug, PartialEq, ZSTConstant)]
#[zst_constant(const_type = "string", const_value = "v2")]
pub struct MyZSTString;

#[derive(Serialize, Deserialize, Debug)]
pub struct StringStruct {
    pub api_version: MyZSTString,
    pub data: String,
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
pub struct CashPaymentMethod;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub payment_method: CashPaymentMethod,
    pub amount: f64,
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
pub struct MyZSTInteger;

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegerStruct {
    pub api_version: MyZSTInteger,
    pub data: String,
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
pub struct MyZSTNumber;

#[derive(Serialize, Deserialize, Debug)]
pub struct NumberStruct {
    pub api_version: MyZSTNumber,
    pub data: String,
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
