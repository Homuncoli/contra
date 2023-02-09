pub mod deserialize;
pub mod deserializer;
pub mod error;
pub mod formatter;
pub mod persistant;
pub mod position;
pub mod serialize;
pub mod serializer;


#[cfg(test)]
mod test {
    use crate::{
        deserialize::{json::FromJson, Deserialize},
        error::SuccessResult,
        position::Position,
        serialize::{json::IntoJson, Serialize},
        serializer::Serializer,
    };

    #[derive(Debug, PartialEq, Eq)]
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

    impl Serialize for PrimitiveDataTypesStruct {
        fn serialize<S: Serializer>(&self, ser: &mut S, _pos: &Position) -> SuccessResult {
            ser.begin_struct("PrimitiveDataTypesStruct", 13)?;

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
            ser.serialize_field("usize", &self.usize, &Position::Trailing)?;
            ser.serialize_field("usize", &self.isize, &Position::Trailing)?;
            ser.serialize_field("string", &self.string, &Position::Closing)?;

            ser.end_struct("PrimitiveDataTypesStruct")?;

            Ok(())
        }
    }

    impl Deserialize for PrimitiveDataTypesStruct {
        fn deserialize<D: crate::deserializer::Deserializer>(
            des: &mut D,
        ) -> Result<Self, crate::error::AnyError> {
            des.deserialize_struct_begin("PrimitiveDataTypesStruct", 13)?;

            let i8 = des.deserialize_field("i8")?;
            let i16 = des.deserialize_field("i16")?;
            let i32 = des.deserialize_field("i32")?;
            let i64 = des.deserialize_field("i64")?;
            let i128 = des.deserialize_field("i128")?;
            let u8 = des.deserialize_field("u8")?;
            let u16 = des.deserialize_field("u16")?;
            let u32 = des.deserialize_field("u32")?;
            let u64 = des.deserialize_field("u64")?;
            let u128 = des.deserialize_field("u128")?;
            let usize = des.deserialize_field("usize")?;
            let isize = des.deserialize_field("usize")?;
            let string = des.deserialize_field("string")?;

            des.deserialize_struct_end("PrimitiveDataTypesStruct")?;

            Ok(Self {
                i8,
                i16,
                i32,
                i64,
                i128,
                u8,
                u16,
                u32,
                u64,
                u128,
                usize,
                isize,
                string,
            })
        }
    }

    #[test]
    fn reconstruction_of_struct_works() {
        let obj = PrimitiveDataTypesStruct::new();

        let json = IntoJson::to_json(&obj);
        assert!(json.is_ok());

        let result = FromJson::from_json(&json.unwrap());
        assert!(result.is_ok());

        assert_eq!(obj, result.unwrap());
    }
}
