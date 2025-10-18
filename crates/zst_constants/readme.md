# Zero-Sized Types Constraints

This crate provides a derive macro [ZSTConstant] that is designed to simplify the creation of Zero-Sized Types (ZST) that represent constant values in json-schema, and in particular in OpenAPI 3.1.x specs. See [here](https://json-schema.org/understanding-json-schema/reference/const) for more information about constants in json-schema.

A ZST is a type that occupies zero bytes of memory at runtime. 
In particular this library leverages that structs can be ZST like
```rust,no_run
struct MyZST;
```
and that they can have methods associated with them. 
In particular we can associate [serde](https://serde.rs/) serialization and deserialization methods.

# Example

Take for example a json-schema like 
```json
{  
    "properties": {
        "version": { 
             "const": "v2"   
        } 
    }
}
```
`version` should serialize and deserialize only to a json like
```json
{
    "version": "v2"
}
```
we can use [ZSTConstant] to define 
```rust,no_run
#[derive(ZSTConstant)]
#[zst_constant(const_type = "string", const_value = "v2")]
struct Version;
```
which expands to 
```rust,no_run
impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringVisitor;

        impl<'de> serde::de::Visitor<'de> for StringVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str()
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value == "v2" { 
                    Ok(Version)
                } else {
                    Err(E::custom(format!("expected string {}, found {}", "v2", value)))
                }
            }
        }
        deserializer.deserialize_str(StringVisitor)
    }
}
impl serde::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    { 
        serializer.serialize_str("v2") 
    }
}
```

you can find more examples in the `tests` folder.