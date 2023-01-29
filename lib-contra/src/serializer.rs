use crate::{error::SuccessResult, serialize::Serialize, position::Position};

macro_rules! decl_serialize_primitive {
    ($type: ident, $ser_func: ident) => {
        fn $ser_func(&mut self, value: &$type) -> SuccessResult;
    };
}

pub trait Serializer {
    fn begin_struct(&mut self, name: &str, fields: usize) -> SuccessResult;
    fn end_struct(&mut self, name: &str, pos: &Position)-> SuccessResult;

    fn serialize_field<V: Serialize>(&mut self, identifier: &str, value: &V, pos: &Position)-> SuccessResult;
    fn serialize_value<V: Serialize>(&mut self, value: &V, pos: &Position) -> SuccessResult;

    decl_serialize_primitive!(i8,    serialize_i8);
    decl_serialize_primitive!(i16,   serialize_i16);
    decl_serialize_primitive!(i32,   serialize_i32);
    decl_serialize_primitive!(i64,   serialize_i64);
    decl_serialize_primitive!(i128,  serialize_i128);
    decl_serialize_primitive!(u8  ,  serialize_u8);
    decl_serialize_primitive!(u16 ,  serialize_u16);
    decl_serialize_primitive!(u32 ,  serialize_u32);
    decl_serialize_primitive!(u64 ,  serialize_u64);
    decl_serialize_primitive!(u128,  serialize_u128);
    decl_serialize_primitive!(usize, serialize_usize);
    decl_serialize_primitive!(isize, serialize_isize);
    decl_serialize_primitive!(String,serialize_string);
}