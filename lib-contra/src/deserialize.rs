use crate::{error::AnyError, deserializer::Deserializer};

pub mod json;

pub trait Deserialize: Sized {
    fn deserialize<D: Deserializer>(des: &mut D) -> Result<Self, AnyError>;
}

macro_rules! impl_deserialize_primitive {
    ($ttype: ident, $des_func: ident) => {
        impl Deserialize for $ttype {
            fn deserialize<D: Deserializer>(des: &mut D) -> Result<$ttype, AnyError> {
                des.$des_func()
            }
        }
    };
}

impl_deserialize_primitive!(i32, deserialize_i32);