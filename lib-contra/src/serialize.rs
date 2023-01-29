use crate::{error::{SuccessResult}, serializer::Serializer, position::Position};

pub mod json;

pub trait Serialize: Sized {
    fn serialize<S: Serializer>(&self, ser: &mut S, pos: &Position) -> SuccessResult;
}

macro_rules! impl_serialize_for_primitive {
    ($type: ident, $ser_func: ident) => {
        impl Serialize for $type {
            fn serialize<S: Serializer>(&self, ser: &mut S, _pos: &Position) -> SuccessResult {
                ser.$ser_func(self)?;

                Ok(())
            }
        }
    };
}

impl_serialize_for_primitive!(i32, serialize_i32);