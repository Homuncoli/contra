use crate::{error::SuccessResult, position::Position, serializer::Serializer};

pub mod json;

pub trait Serialize: Sized {
    fn serialize<S: Serializer>(&self, ser: &mut S, pos: &Position) -> SuccessResult;
}

macro_rules! impl_serialize_primitive {
    ($type: ident, $ser_func: ident) => {
        impl Serialize for $type {
            fn serialize<S: Serializer>(&self, ser: &mut S, _pos: &Position) -> SuccessResult {
                ser.$ser_func(self)?;

                Ok(())
            }
        }
    };
}

impl_serialize_primitive!(i8, serialize_i8);
impl_serialize_primitive!(i16, serialize_i16);
impl_serialize_primitive!(i32, serialize_i32);
impl_serialize_primitive!(i64, serialize_i64);
impl_serialize_primitive!(i128, serialize_i128);
impl_serialize_primitive!(u8, serialize_u8);
impl_serialize_primitive!(u16, serialize_u16);
impl_serialize_primitive!(u32, serialize_u32);
impl_serialize_primitive!(u64, serialize_u64);
impl_serialize_primitive!(u128, serialize_u128);
impl_serialize_primitive!(f32, serialize_f32);
impl_serialize_primitive!(f64, serialize_f64);
impl_serialize_primitive!(usize, serialize_usize);
impl_serialize_primitive!(isize, serialize_isize);
impl_serialize_primitive!(String, serialize_str);

impl Serialize for &str {
    fn serialize<S: Serializer>(&self, ser: &mut S, _pos: &Position) -> SuccessResult {
        ser.serialize_str(self)
    }
}

impl<Item: Serialize> Serialize for Vec<Item> {
    fn serialize<S: Serializer>(&self, ser: &mut S, _pos: &Position) -> SuccessResult {
        let len = self.len();
        ser.begin_collection(stringify!(Vec<Item>), len)?;

        let mut iter = self.iter();
        let closing_item = iter.next_back();
        for (i, trailing_item) in iter.enumerate() {
            ser.serialize_item(i, trailing_item, &Position::Trailing)?;
        }
        if closing_item.is_some() {
            ser.serialize_item(self.len() - 1, closing_item.unwrap(), &Position::Closing)?;
        }

        ser.end_collection(stringify!(Vec<Item>))?;

        Ok(())
    }
}
