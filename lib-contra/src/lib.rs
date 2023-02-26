//! Function implementation for [contra](https://docs.rs/contra)
//!
//! Provides the function and traits needed for the serialization and deserialization of any arbitrary object.

pub mod deserialize;
pub mod error;
pub mod formatter;
pub mod persistent;
pub mod position;
pub mod serialize;

#[cfg(test)]
mod test {
    use crate::{
        error::SuccessResult,
        position::Position,
        serialize::{Serialize, Serializer}, deserialize::{Deserialize, Visitor}
    };

    #[derive(Debug, PartialEq)]
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
        f32: f32,
        f64: f64,
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
                f32: f32::MAX,
                f64: f64::MAX,
                usize: usize::MAX,
                isize: isize::MAX,
                string: "Hello World!".to_string(),
            }
        }
    }

    impl Serialize for PrimitiveDataTypesStruct {
        fn serialize<S: Serializer>(&self, ser: &mut S, _pos: &Position) -> SuccessResult {
            ser.begin_struct("PrimitiveDataTypesStruct", 15)?;

            ser.serialize_field("i8", &self.i8, &Position::Trailing)?;
            ser.serialize_field("i16", &self.i16, &Position::Trailing)?;
            ser.serialize_field("i32", &self.i32, &Position::Trailing)?;
            ser.serialize_field("i64", &self.i64, &Position::Trailing)?;
            ser.serialize_field("i128", &self.i128, &Position::Trailing)?;
            ser.serialize_field("u8", &self.u8, &Position::Trailing)?;
            ser.serialize_field("u16", &self.u16, &Position::Trailing)?;
            ser.serialize_field("u32", &self.u32, &Position::Trailing)?;
            ser.serialize_field("u64", &self.u64, &Position::Trailing)?;
            ser.serialize_field("u128", &self.u128, &Position::Trailing)?;
            ser.serialize_field("f32", &self.f32, &Position::Trailing)?;
            ser.serialize_field("f64", &self.f64, &Position::Trailing)?;
            ser.serialize_field("usize", &self.usize, &Position::Trailing)?;
            ser.serialize_field("usize", &self.isize, &Position::Trailing)?;
            ser.serialize_field("string", &self.string, &Position::Closing)?;

            ser.end_struct("PrimitiveDataTypesStruct")?;

            Ok(())
        }
    }

    impl Deserialize for PrimitiveDataTypesStruct {
        fn deserialize<D: crate::deserialize::Deserializer>(de: D) -> Result<Self, crate::error::AnyError> {
            enum Field {
                I8,
                I16,
                I32,
                I64,
                I128,
                U8,
                U16,
                U32,
                U64,
                U128,
                F32,
                F64,
                Usize,
                Isize,
                String,
            }
            impl Deserialize for Field {
                fn deserialize<D: crate::deserialize::Deserializer>(de: D) -> Result<Self, crate::error::AnyError> {
                    struct FieldVisitor {}
                    impl Visitor for FieldVisitor {
                        type Value = Field;

                        fn expected_a(self) -> String {
                            "PrimitiveDataTypesStruct field".to_string()
                        }

                        fn visit_str(self, v: &str) -> Result<Self::Value, crate::error::AnyError> {
                            match v {
                                "i8" => Ok(Field::I8),
                                "i16" => Ok(Field::I16),
                                "i32" => Ok(Field::I32),
                                "i64" => Ok(Field::I64),
                                "i128" => Ok(Field::I128),
                                "u8" => Ok(Field::U8),
                                "u16" => Ok(Field::U16),
                                "u32" => Ok(Field::U32),
                                "u64" => Ok(Field::U64),
                                "u128" => Ok(Field::U128),
                                "f32" => Ok(Field::F32),
                                "f64" => Ok(Field::F64),
                                "usize" => Ok(Field::Usize),
                                "isize" => Ok(Field::Isize),
                                "string" => Ok(Field::String),
                                val => Err(format!("unknown field {}", val).into())
                            }
                        }
                    }
                    de.deserialize_str(FieldVisitor {})
                }
            }

            struct PrimitiveDataTypesStructVisitor {}
            impl Visitor for PrimitiveDataTypesStructVisitor {
                type Value = PrimitiveDataTypesStruct;

                fn expected_a(self) -> String {
                    "PrimitiveDataTypesStruct".to_string()
                }

                fn visit_map<M: crate::deserialize::MapAccess>(self, mut map: M) -> Result<Self::Value, crate::error::AnyError> {
                    let mut i8 = None; 
                    let mut i16 = None; 
                    let mut i32 = None; 
                    let mut i64 = None; 
                    let mut i128 = None; 
                    let mut u8 = None; 
                    let mut u16 = None; 
                    let mut u32 = None; 
                    let mut u64 = None; 
                    let mut u128 = None; 
                    let mut f32 = None; 
                    let mut f64 = None; 
                    let mut usize = None; 
                    let mut isize = None; 
                    let mut string = None;

                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::I8 => { if i8.is_some() { return Err("duplicate field i8".into() )} i8 = Some(map.next_value()?) }
                            Field::I16 => { if i16.is_some() { return Err("duplicate field i16".into() )} i16 = Some(map.next_value()?) }
                            Field::I32 => { if i32.is_some() { return Err("duplicate field i32".into() )} i32 = Some(map.next_value()?) }
                            Field::I64 => { if i64.is_some() { return Err("duplicate field i64".into() )} i64 = Some(map.next_value()?) }
                            Field::I128 => { if i128.is_some() { return Err("duplicate field i128".into() )} i128 = Some(map.next_value()?) }
                            Field::U8 => { if u8.is_some() { return Err("duplicate field u8".into() )} u8 = Some(map.next_value()?) }
                            Field::U16 => { if u16.is_some() { return Err("duplicate field u16".into() )} u16 = Some(map.next_value()?) }
                            Field::U32 => { if u32.is_some() { return Err("duplicate field u32".into() )} u32 = Some(map.next_value()?) }
                            Field::U64 => { if u64.is_some() { return Err("duplicate field u64".into() )} u64 = Some(map.next_value()?) }
                            Field::U128 => { if u128.is_some() { return Err("duplicate field u128".into() )} u128 = Some(map.next_value()?) }
                            Field::F32 => { if f32.is_some() { return Err("duplicate field f32".into() )} f32 = Some(map.next_value()?) }
                            Field::F64 => { if f64.is_some() { return Err("duplicate field f64".into() )} f64 = Some(map.next_value()?) }
                            Field::Usize => { if usize.is_some() { return Err("duplicate field usize".into() )} usize = Some(map.next_value()?) }
                            Field::Isize => { if isize.is_some() { return Err("duplicate field isize".into() )} isize = Some(map.next_value()?) }
                            Field::String => { if string.is_some() { return Err("duplicate field string".into() )} string = Some(map.next_value()?) }
                        }
                    };

                    let i8 = i8.ok_or_else(|| "missing field i8")?;
                    let i16 = i16.ok_or_else(|| "missing field i16")?;
                    let i32 = i32.ok_or_else(|| "missing field i32")?;
                    let i64 = i64.ok_or_else(|| "missing field i64")?;
                    let i128 = i128.ok_or_else(|| "missing field i128")?;
                    let u8 = u8.ok_or_else(|| "missing field u8")?;
                    let u16 = u16.ok_or_else(|| "missing field u16")?;
                    let u32 = u32.ok_or_else(|| "missing field u32")?;
                    let u64 = u64.ok_or_else(|| "missing field u64")?;
                    let u128 = u128.ok_or_else(|| "missing field u128")?;
                    let f32 = f32.ok_or_else(|| "missing field f32")?;
                    let f64 = f64.ok_or_else(|| "missing field f64")?;
                    let usize = usize.ok_or_else(|| "missing field usize")?;
                    let isize = isize.ok_or_else(|| "missing field isize")?;
                    let string = string.ok_or_else(|| "missing field string")?;

                    Ok(PrimitiveDataTypesStruct {
                        i8: i8,
                        i16: i16,
                        i32: i32,
                        i64: i64,
                        i128: i128,
                        u8: u8,
                        u16: u16,
                        u32: u32,
                        u64: u64,
                        u128: u128,
                        f32: f32,
                        f64: f64,
                        usize: usize,
                        isize: isize,
                        string: string,
                    })
                }
            }

            de.deserialize_map(PrimitiveDataTypesStructVisitor {})
        }
    }

    // #[test]
    // fn reconstruction_of_struct_works() {
    //     let obj = PrimitiveDataTypesStruct::new();

    //     let json = IntoJson::to_json(&obj);
    //     assert!(json.is_ok());

    //     let result = FromJson::from_json(&json.unwrap());
    //     assert!(result.is_ok());

    //     assert_eq!(obj, result.unwrap());
    // }
}
