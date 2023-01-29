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

impl_deserialize_primitive!(i8,    deserialize_i8);
impl_deserialize_primitive!(i16,   deserialize_i16);
impl_deserialize_primitive!(i32,   deserialize_i32);
impl_deserialize_primitive!(i64,   deserialize_i64);
impl_deserialize_primitive!(i128,  deserialize_i128);
impl_deserialize_primitive!(u8  ,  deserialize_u8);
impl_deserialize_primitive!(u16 ,  deserialize_u16);
impl_deserialize_primitive!(u32 ,  deserialize_u32);
impl_deserialize_primitive!(u64 ,  deserialize_u64);
impl_deserialize_primitive!(u128,  deserialize_u128);
impl_deserialize_primitive!(usize, deserialize_usize);
impl_deserialize_primitive!(isize, deserialize_isize);
impl_deserialize_primitive!(String,deserialize_string);

impl<Item: Deserialize> Deserialize for Vec<Item> {
    fn deserialize<D: Deserializer>(des: &mut D) -> Result<Self, AnyError> {
        des.deserialize_vec(stringify!(Vec<Item>))
    }
}