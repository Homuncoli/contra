use crate::{error::{SuccessResult}, serializer::Serializer, position::Position};

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

impl_serialize_primitive!(i8  ,  serialize_i8);
impl_serialize_primitive!(i16 ,  serialize_i16);
impl_serialize_primitive!(i32 ,  serialize_i32);
impl_serialize_primitive!(i64 ,  serialize_i64);
impl_serialize_primitive!(i128,  serialize_i128);
impl_serialize_primitive!(u8  ,  serialize_u8);
impl_serialize_primitive!(u16 ,  serialize_u16);
impl_serialize_primitive!(u32 ,  serialize_u32);
impl_serialize_primitive!(u64 ,  serialize_u64);
impl_serialize_primitive!(u128,  serialize_u128);
impl_serialize_primitive!(usize, serialize_usize);
impl_serialize_primitive!(isize, serialize_isize);
impl_serialize_primitive!(String,serialize_string);