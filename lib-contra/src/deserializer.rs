use crate::{error::{SuccessResult, AnyError}, deserialize::Deserialize};


macro_rules! decl_deserialize_primitive {
    ($ttype: ident, $des_func: ident) => {
        fn $des_func(&mut self) -> Result<$ttype, AnyError>;
    };
}

pub trait Deserializer {
    fn deserialize_struct_begin(&mut self, name: &str, fields: usize) -> SuccessResult;
    fn deserialize_struct_end(&mut self, name: &str) -> SuccessResult;

    fn deserialize_field<T: Deserialize>(&mut self, field: &str) -> Result<T, AnyError>; 

    decl_deserialize_primitive!(i32, deserialize_i32);
}