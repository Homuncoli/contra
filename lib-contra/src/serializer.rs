use crate::{error::SuccessResult, serialize::Serialize};

macro_rules! decl_serialize_primitive {
    ($type: ident, $ser_func: ident) => {
        fn $ser_func(&mut self, value: &$type) -> SuccessResult;
    };
}

pub trait Serializer {
    fn begin_struct(&mut self, name: &str, fields: usize) -> SuccessResult;
    fn end_struct(&mut self, name: &str)-> SuccessResult;

    fn serialize_field<V: Serialize>(&mut self, identifier: &str, value: &V)-> SuccessResult;
    fn serialize_value<V: Serialize>(&mut self, value: &V) -> SuccessResult;

    decl_serialize_primitive!(i32, serialize_i32);
}