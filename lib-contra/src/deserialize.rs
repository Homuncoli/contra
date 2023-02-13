use crate::{deserializer::Deserializer, error::AnyError};

pub mod json;

/// Allows for the deserialization of the implemented type
///
/// Implementors must provide the functionality to construct *Self* into from any [Deserializer].
///
/// # Example
/// Best to not implemented by hand but rather derived via the Deserialize derive macro of the [proc_contra](https://docs.rs/proc_contra/) crate.
/// See: [Contra](https://docs.rs/contra/)
/// ```
/// use crate::{lib_contra::{deserialize::Deserialize, position::Position, deserializer::Deserializer, error::AnyError}};
/// 
/// struct Point {
///     x: f32,
///     y: f32,
///     z: f32
/// }
///
/// impl Deserialize for Point {
///     fn deserialize<D: Deserializer>(des: &mut D) -> Result<Self, AnyError> {
///         des.deserialize_struct_begin("Point", 3)?;
///     
///         let x = des.deserialize_field("x")?;
///         let y = des.deserialize_field("y")?;
///         let z = des.deserialize_field("z")?;
/// 
///         des.deserialize_struct_end("Point")?;
///         Ok(Self { x, y, z })
///     }
/// }
/// ```
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

impl_deserialize_primitive!(i8, deserialize_i8);
impl_deserialize_primitive!(i16, deserialize_i16);
impl_deserialize_primitive!(i32, deserialize_i32);
impl_deserialize_primitive!(i64, deserialize_i64);
impl_deserialize_primitive!(i128, deserialize_i128);
impl_deserialize_primitive!(u8, deserialize_u8);
impl_deserialize_primitive!(u16, deserialize_u16);
impl_deserialize_primitive!(u32, deserialize_u32);
impl_deserialize_primitive!(u64, deserialize_u64);
impl_deserialize_primitive!(u128, deserialize_u128);
impl_deserialize_primitive!(f32, deserialize_f32);
impl_deserialize_primitive!(f64, deserialize_f64);
impl_deserialize_primitive!(usize, deserialize_usize);
impl_deserialize_primitive!(isize, deserialize_isize);
impl_deserialize_primitive!(String, deserialize_string);

impl<Item: Deserialize> Deserialize for Vec<Item> {
    fn deserialize<D: Deserializer>(des: &mut D) -> Result<Self, AnyError> {
        des.deserialize_vec(stringify!(Vec<Item>))
    }
}
