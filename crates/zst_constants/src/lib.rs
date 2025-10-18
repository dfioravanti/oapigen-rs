#![doc = include_str!("../readme.md")]

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use serde_json::Value;
use syn::{DeriveInput, Ident, LitBool, LitFloat, LitInt, LitStr, parse_macro_input};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(zst_constant), forward_attrs(allow, doc, cfg))]
struct ConstMacroArgs {
    const_type: LitStr,
    const_value: Option<LitStr>,
}

/// test
#[proc_macro_derive(ZSTConstant, attributes(zst_constant))]
pub fn as_constant(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let args = match ConstMacroArgs::from_derive_input(&ast) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let struct_name = &ast.ident;

    let maybe_output = match args.const_type.value().as_str() {
        "none" | "null" => process_none(struct_name),
        "bool" => process_bool(args.const_value, struct_name),
        "string" => process_string(args.const_value, struct_name),
        "integer" | "int" => process_int(args.const_value, struct_name),
        "float" | "doable" | "number" => process_number(args.const_value, struct_name),
        "array" => process_struct(args.const_value, struct_name),
        "struct" | "object" => process_struct(args.const_value, struct_name),
        _ => {
            return syn::Error::new_spanned(args.const_type, "Unknown const_type. ")
                .to_compile_error()
                .into();
        }
    };

    let output = match maybe_output {
        Ok(value) => value,
        Err(e) => return e.to_compile_error().into(),
    };

    TokenStream::from(output)
}

fn require_const_value(maybe_const_value: Option<LitStr>) -> Result<LitStr, syn::Error> {
    match maybe_const_value {
        Some(v) => Ok(v),
        None => Err(syn::Error::new_spanned(
            maybe_const_value,
            "Missing field `const_type`",
        )),
    }
}

fn process_none(original_name: &Ident) -> Result<TokenStream2, syn::Error> {
    let output = quote! {
        impl<'de> serde::Deserialize<'de> for #original_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let constant = serde_json::Value::Null;
                let incoming  = serde_json::Value::deserialize(deserializer)?;

                if incoming.eq(&constant){
                    Ok(#original_name)
                } else {
                    Err(serde::de::Error::custom(format!("expected {}, found {}", constant, incoming)))
                }
            }
        }

        impl serde::Serialize for #original_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serde_json::Value::Null.serialize(serializer)
            }
        }
    };

    Ok(output)
}

fn process_bool(
    maybe_const_value: Option<LitStr>,
    original_name: &Ident,
) -> Result<TokenStream2, syn::Error> {
    let string_lit = require_const_value(maybe_const_value)?;
    let bool_lit = string_lit.parse::<LitBool>()?;

    let output = quote! {
        impl<'de> serde::Deserialize<'de> for #original_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct BoolVisitor;

                impl<'de> serde::de::Visitor<'de> for BoolVisitor {
                    type Value = #original_name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(#string_lit)
                    }

                    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        if value == #bool_lit {
                            Ok(#original_name)
                        } else {
                            Err(E::custom(format!("expected bool {}, found {}", #bool_lit, value)))
                        }
                    }
                }

                deserializer.deserialize_bool(BoolVisitor)
            }
        }

        impl serde::Serialize for #original_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_bool(#bool_lit)
            }
        }
    };

    Ok(output)
}

fn process_string(
    maybe_const_value: Option<LitStr>,
    original_name: &Ident,
) -> Result<TokenStream2, syn::Error> {
    let string_lit = require_const_value(maybe_const_value)?;
    let output = quote! {
        impl<'de> serde::Deserialize<'de> for #original_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct StringVisitor;

                impl<'de> serde::de::Visitor<'de> for StringVisitor {
                    type Value = #original_name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(#string_lit)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        if value == #string_lit {
                            Ok(#original_name)
                        } else {
                            Err(E::custom(format!("expected string {}, found {}", #string_lit, value)))
                        }
                    }
                }

                deserializer.deserialize_str(StringVisitor)
            }
        }

        impl serde::Serialize for #original_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(#string_lit)
            }
        }
    };

    Ok(output)
}

fn process_int(
    maybe_const_value: Option<LitStr>,
    original_name: &Ident,
) -> Result<TokenStream2, syn::Error> {
    let string_lit = require_const_value(maybe_const_value)?;
    let int_lit = string_lit.parse::<LitInt>()?;
    let constant_value = int_lit.base10_parse::<i64>()?;

    let output = quote! {
        impl<'de> serde::Deserialize<'de> for #original_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct IntVisitor;

                impl<'de> serde::de::Visitor<'de> for IntVisitor {
                    type Value = #original_name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(#string_lit)
                    }

                    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        if value == #constant_value {
                            Ok(#original_name)
                        } else {
                            Err(E::custom(format!("expected integer {}, found {}", #constant_value, value)))
                        }
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        self.visit_i64(value as i64)
                    }
                }

                deserializer.deserialize_u64(IntVisitor)
            }
        }

        impl serde::Serialize for #original_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_i64(#constant_value)
            }
        }
    };

    Ok(output)
}

fn process_number(
    maybe_const_value: Option<LitStr>,
    original_name: &Ident,
) -> Result<TokenStream2, syn::Error> {
    let string_lit = require_const_value(maybe_const_value)?;
    let float_lit = string_lit.parse::<LitFloat>()?;
    let constant_value = float_lit.base10_parse::<f64>()?;

    let output = quote! {
        impl<'de> serde::Deserialize<'de> for #original_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct NumberVisitor;

                impl<'de> serde::de::Visitor<'de> for NumberVisitor {
                    type Value = #original_name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(#string_lit)
                    }

                    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        if value == #constant_value {
                            Ok(#original_name)
                        } else {
                            Err(E::custom(format!("expected number {}, found {}", #constant_value, value)))
                        }
                    }
                }

                deserializer.deserialize_f64(NumberVisitor)
            }
        }

        impl serde::Serialize for #original_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_f64(#constant_value)
            }
        }
    };

    Ok(output)
}

fn process_struct(
    maybe_const_value: Option<LitStr>,
    original_name: &Ident,
) -> Result<TokenStream2, syn::Error> {
    let string_lit = require_const_value(maybe_const_value)?;
    let json_str = string_lit.value();
    let _: Value = match serde_json::from_str(&json_str) {
        Ok(v) => v,
        Err(e) => {
            let error_msg = format!("Failed to parse JSON: {}", e);
            return Err(syn::Error::new_spanned(string_lit, error_msg));
        }
    };

    let constant_value = string_lit.to_token_stream();

    let output = quote! {
        impl<'de> serde::Deserialize<'de> for #original_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let constant: serde_json::Value = serde_json::from_str(#constant_value).unwrap();
                let incoming  = serde_json::Value::deserialize(deserializer)?;

                if incoming.eq(&constant){
                    Ok(#original_name)
                } else {
                    Err(serde::de::Error::custom(format!("expected {}, found {}", constant, incoming)))
                }
            }
        }

        impl serde::Serialize for #original_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let constant: serde_json::Value = serde_json::from_str(#constant_value).unwrap();
                constant.serialize(serializer)
            }
        }
    };

    Ok(output)
}
