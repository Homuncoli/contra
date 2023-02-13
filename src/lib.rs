//! Lightweight and easy to use serialization, deserialization.
//!
//! Provides abstract serialization into specific formats.
//! Additionally provides the functionality to save and load the serialized content directly from and to disk.
//!
//! To implement more data formats see: [Serializer](self::lib_contra::serializer::Serializer), [Deserializer](self::lib_contra::deserializer::Deserializer)
//!
//! # Examples
//! ```
//! use contra::{Serialize, Deserialize};
//! use lib_contra::persistent::Persistent;
//! 
//! #[derive(Serialize, Deserialize)]
//! struct Point {
//!     x: f32,
//!     y: f32,
//!     z: f32
//! }
//!
//! fn modify_point() -> Result<(), Box<dyn std::error::Error>> {
//!     let p = Point::load("path/to/point.json")?;
//!     assert_eq!(p.x, 1.0f32);
//!     p.x = 2.0f32;
//!     p.save("path/to/point.json")?;
//!     Ok(())
//! }
//! ```

pub use lib_contra::{
    self,
    deserialize::{self, json::FromJson, Deserialize},
    serialize::{self, json::IntoJson, Serialize},
};
pub use proc_contra::{Deserialize, Serialize};

#[cfg(test)]
mod test {
    use lib_contra::{deserialize::json::FromJson, serialize::json::IntoJson};
    use proc_contra::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct EmptyStruct {}

    #[test]
    fn empty_struct_works() {
        let expected = EmptyStruct {};

        let json = IntoJson::to_json(&expected);
        assert!(json.is_ok());

        let result = FromJson::from_json(&json.unwrap());
        assert!(result.is_ok());

        assert_eq!(expected, result.unwrap());
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct PrimitiveDataTypesStruct {
        i8: i32,
        i16: i16,
        i32: i32,
        i64: i64,
        i128: i128,
        u8: u32,
        u16: u16,
        u32: u32,
        u64: u64,
        u128: u128,
        usize: usize,
        isize: isize,
        string: String,
    }

    impl PrimitiveDataTypesStruct {
        fn new() -> Self {
            PrimitiveDataTypesStruct {
                i8: 1000,
                i16: i16::MAX,
                i32: i32::MAX,
                i64: i64::MAX,
                i128: i128::MAX,
                u8: 1000,
                u16: u16::MAX,
                u32: u32::MAX,
                u64: u64::MAX,
                u128: u128::MAX,
                usize: usize::MAX,
                isize: isize::MAX,
                string: "Hello World!".to_string(),
            }
        }
    }

    #[test]
    fn primitive_data_struct_works() {
        let expected = PrimitiveDataTypesStruct::new();

        let json = IntoJson::to_json(&expected);
        assert!(json.is_ok());

        let result = FromJson::from_json(&json.unwrap());
        assert!(result.is_ok());

        assert_eq!(expected, result.unwrap());
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct NestedDataStruct {
        p1: PrimitiveDataTypesStruct,
        p2: PrimitiveDataTypesStruct,
    }

    #[test]
    fn nested_data_struct_works() {
        let expected = NestedDataStruct {
            p1: PrimitiveDataTypesStruct::new(),
            p2: PrimitiveDataTypesStruct::new(),
        };

        let json = IntoJson::to_json(&expected);
        assert!(json.is_ok());
        let json = json.unwrap();

        let result = FromJson::from_json(&json);
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn vec_string_works() {
        let expected = vec!["A", "B", "C"];

        let json = IntoJson::to_json(&expected);
        assert!(json.is_ok());
        let json = json.unwrap();

        let result = FromJson::from_json(&json);
        assert!(result.is_ok());
        let result: Vec<String> = result.unwrap();

        assert_eq!(expected, result);
    }
}
