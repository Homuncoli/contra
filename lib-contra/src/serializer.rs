use crate::{error::SuccessResult, position::Position, serialize::Serialize};

macro_rules! decl_serialize_primitive {
    ($type: ident, $ser_func: ident) => {
        fn $ser_func(&mut self, value: &$type) -> SuccessResult;
    };
}

/// Handles the serialization and formatting of an arbitrary serializable
///
/// Any Serializer must implement methods that allow for the serialization of all supported data types.
/// These methods are then called in the implementation (most often derived) of the [Serialize](crate::serialize::Serialize) trait.
pub trait Serializer {
    fn begin_struct(&mut self, name: &str, fields: usize) -> SuccessResult;
    fn end_struct(&mut self, name: &str) -> SuccessResult;

    fn begin_collection(&mut self, name: &str, size: usize) -> SuccessResult;
    fn end_collection(&mut self, name: &str) -> SuccessResult;

    fn serialize_item<V: Serialize>(&mut self, i: usize, item: &V, pos: &Position)
        -> SuccessResult;

    fn serialize_field<V: Serialize>(
        &mut self,
        identifier: &str,
        value: &V,
        pos: &Position,
    ) -> SuccessResult;
    fn serialize_value<V: Serialize>(&mut self, value: &V, pos: &Position) -> SuccessResult;

    fn serialize_str(&mut self, value: &str) -> SuccessResult;

    decl_serialize_primitive!(i8, serialize_i8);
    decl_serialize_primitive!(i16, serialize_i16);
    decl_serialize_primitive!(i32, serialize_i32);
    decl_serialize_primitive!(i64, serialize_i64);
    decl_serialize_primitive!(i128, serialize_i128);
    decl_serialize_primitive!(u8, serialize_u8);
    decl_serialize_primitive!(u16, serialize_u16);
    decl_serialize_primitive!(u32, serialize_u32);
    decl_serialize_primitive!(u64, serialize_u64);
    decl_serialize_primitive!(u128, serialize_u128);
    decl_serialize_primitive!(f32, serialize_f32);
    decl_serialize_primitive!(f64, serialize_f64);
    decl_serialize_primitive!(usize, serialize_usize);
    decl_serialize_primitive!(isize, serialize_isize);
}
