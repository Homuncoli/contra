use crate::{
    deserialize::Deserialize,
    error::{AnyError, SuccessResult},
};

macro_rules! decl_deserialize_primitive {
    ($ttype: ident, $des_func: ident) => {
        fn $des_func(&mut self) -> Result<$ttype, AnyError>;
    };
}

pub trait Deserializer {
    fn deserialize_struct_begin(&mut self, name: &str, fields: usize) -> SuccessResult;
    fn deserialize_struct_end(&mut self, name: &str) -> SuccessResult;

    fn deserialize_vec<Item: Deserialize>(&mut self, name: &str) -> Result<Vec<Item>, AnyError>;

    fn deserialize_field<T: Deserialize>(&mut self, field: &str) -> Result<T, AnyError>;

    decl_deserialize_primitive!(i8, deserialize_i8);
    decl_deserialize_primitive!(i16, deserialize_i16);
    decl_deserialize_primitive!(i32, deserialize_i32);
    decl_deserialize_primitive!(i64, deserialize_i64);
    decl_deserialize_primitive!(i128, deserialize_i128);
    decl_deserialize_primitive!(u8, deserialize_u8);
    decl_deserialize_primitive!(u16, deserialize_u16);
    decl_deserialize_primitive!(u32, deserialize_u32);
    decl_deserialize_primitive!(u64, deserialize_u64);
    decl_deserialize_primitive!(u128, deserialize_u128);
    decl_deserialize_primitive!(usize, deserialize_usize);
    decl_deserialize_primitive!(isize, deserialize_isize);
    decl_deserialize_primitive!(String, deserialize_string);
}
