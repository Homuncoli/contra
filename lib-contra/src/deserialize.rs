pub mod json;

use std::{
    collections::HashMap,
    hash::Hash,
    io::{self, ErrorKind},
    marker::PhantomData,
};

use crate::error::AnyError;

use self::error::visiting_but_expected;

macro_rules! decl_deserialize_primitive {
    ($deserialize_fn: ident) => {
        fn $deserialize_fn<V: Visitor>(self, v: V) -> Result<V::Value, AnyError>;
    };
}

macro_rules! decl_visit_primitive {
    ($ttype: ident, $visit_fn: ident) => {
        fn $visit_fn(self, _v: $ttype) -> Result<Self::Value, AnyError> {
            Err(visiting_but_expected(stringify!($ttype), &self.expected_a()).into())
        }
    };
}

macro_rules! impl_deserialize_primitive {
    ($ttype: ident, $visit_fn: ident, $deserialize_fn: ident) => {
        impl Deserialize for $ttype {
            fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError> {
                struct PrimitiveVisitor {}
                impl Visitor for PrimitiveVisitor {
                    type Value = $ttype;

                    fn expected_a(self) -> String {
                        stringify!($ttype).to_string()
                    }

                    fn $visit_fn(self, val: $ttype) -> Result<Self::Value, AnyError> {
                        Ok(val)
                    }
                }

                des.$deserialize_fn(PrimitiveVisitor {})
            }
        }
    };
}

/// Implementors of this trait can be deserialized from any format
pub trait Deserialize: Sized {
    fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError>;
}

/// Parses bytes and delegates them to a visitor
pub trait Deserializer: Sized {
    fn deserialize_map<V: Visitor>(self, v: V) -> Result<V::Value, AnyError>;
    fn deserialize_seq<V: Visitor>(self, v: V) -> Result<V::Value, AnyError>;
    fn deserialize_struct<V: Visitor>(self, v: V) -> Result<V::Value, AnyError>;
    fn deserialize_str<V: Visitor>(self, v: V) -> Result<V::Value, AnyError>;

    decl_deserialize_primitive!(deserialize_i8);
    decl_deserialize_primitive!(deserialize_i16);
    decl_deserialize_primitive!(deserialize_i32);
    decl_deserialize_primitive!(deserialize_i64);
    decl_deserialize_primitive!(deserialize_i128);
    decl_deserialize_primitive!(deserialize_u8);
    decl_deserialize_primitive!(deserialize_u16);
    decl_deserialize_primitive!(deserialize_u32);
    decl_deserialize_primitive!(deserialize_u64);
    decl_deserialize_primitive!(deserialize_u128);
    decl_deserialize_primitive!(deserialize_f32);
    decl_deserialize_primitive!(deserialize_f64);
    decl_deserialize_primitive!(deserialize_isize);
    decl_deserialize_primitive!(deserialize_usize);
}

/// Maps a parsed value to a Rust type
pub trait Visitor: Sized {
    type Value;
    fn expected_a(self) -> String;

    fn visit_map<M: MapAccess>(self, _map: M) -> Result<Self::Value, AnyError> {
        Err(visiting_but_expected("map", &self.expected_a()).into())
    }
    fn visit_seq<S: SeqAccess>(self, _seq: S) -> Result<Self::Value, AnyError> {
        Err(visiting_but_expected("seq", &self.expected_a()).into())
    }
    fn visit_str(self, _v: &str) -> Result<Self::Value, AnyError> {
        Err(visiting_but_expected("str", &self.expected_a()).into())
    }

    decl_visit_primitive!(i8, visit_i8);
    decl_visit_primitive!(i16, visit_i16);
    decl_visit_primitive!(i32, visit_i32);
    decl_visit_primitive!(i64, visit_i64);
    decl_visit_primitive!(i128, visit_i128);
    decl_visit_primitive!(u8, visit_u8);
    decl_visit_primitive!(u16, visit_u16);
    decl_visit_primitive!(u32, visit_u32);
    decl_visit_primitive!(u64, visit_u64);
    decl_visit_primitive!(u128, visit_u128);
    decl_visit_primitive!(f32, visit_f32);
    decl_visit_primitive!(f64, visit_f64);
    decl_visit_primitive!(usize, visit_usize);
    decl_visit_primitive!(isize, visit_isize);
}

/// Allows the access to key-value pairs
pub trait MapAccess {
    fn next_value<V: Deserialize>(&mut self) -> Result<V, AnyError>;
    fn next_key<K: Deserialize>(&mut self) -> Result<Option<K>, AnyError>;
}

/// Allows the access to sequences
pub trait SeqAccess {
    fn next_value<V: Deserialize>(&mut self) -> Result<Option<V>, AnyError>;
}

// ##########################################

mod error {
    pub(crate) fn visiting_but_expected(etype: &str, gtype: &str) -> String {
        "visiting a ".to_string() + etype + " but expected a " + gtype
    }
}

// ##########################################

/// Utility functions for io::Read and io::Seek
pub trait Peek {
    fn peek(&mut self) -> Result<Option<u8>, AnyError>;
    fn read_until(&mut self, end: &[u8]) -> Result<Vec<u8>, AnyError>;
    fn consume(&mut self) -> Result<(), AnyError>;
    fn consume_matching(&mut self, matches: &[u8]) -> Result<(), AnyError>;
}

impl<R: io::Read + io::Seek> Peek for R {
    fn peek(&mut self) -> Result<Option<u8>, AnyError> {
        let start = self.stream_position()?;
        let mut char: [u8; 1] = [0];
        let char = match self.read_exact(&mut char) {
            Ok(_) => Ok(Some(char)),
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => Ok(None),
            Err(err) => Err(err),
        };
        self.seek(io::SeekFrom::Start(start))?;
        char.map(|o| {
            if o.is_some() {
                return Some(*o.unwrap().get(0).unwrap());
            }
            return None;
        })
        .map_err(|e| e.into())
    }

    fn consume(&mut self) -> Result<(), AnyError> {
        self.seek(io::SeekFrom::Current(1))?;
        Ok(())
    }

    fn consume_matching(&mut self, matches: &[u8]) -> Result<(), AnyError> {
        loop {
            let char = self.peek()?;
            if char.is_some() && matches.contains(&char.unwrap()) {
                self.consume()?;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn read_until(&mut self, end: &[u8]) -> Result<Vec<u8>, AnyError> {
        let mut vec = vec![];
        loop {
            let char = self.peek()?;
            if char.is_some() && !end.contains(&char.unwrap()) {
                self.consume()?;
                vec.push(char.unwrap());
            } else {
                break;
            }
        }
        Ok(vec)
    }
}

// ##########################################
impl Deserialize for String {
    fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError> {
        struct StringVisitor {}
        impl Visitor for StringVisitor {
            type Value = String;

            fn expected_a(self) -> String {
                "string".to_string()
            }

            fn visit_str(self, v: &str) -> Result<Self::Value, AnyError> {
                Ok(v.to_string())
            }
        }

        des.deserialize_str(StringVisitor {})
    }
}

impl<I: Deserialize> Deserialize for Vec<I> {
    fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError> {
        struct VecVisitor<I> {
            marker: PhantomData<I>,
        }
        impl<I: Deserialize> Visitor for VecVisitor<I> {
            type Value = Vec<I>;

            fn expected_a(self) -> String {
                "vec".to_string()
            }

            fn visit_seq<S: SeqAccess>(self, mut seq: S) -> Result<Self::Value, AnyError> {
                let mut vec = vec![];

                while let Some(item) = seq.next_value()? {
                    vec.push(item);
                }

                Ok(vec)
            }
        }

        des.deserialize_seq(VecVisitor {
            marker: PhantomData::<I>,
        })
    }
}

impl<K: Deserialize + Hash + Eq, V: Deserialize> Deserialize for HashMap<K, V> {
    fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError> {
        struct HashMapVisitor<K, V> {
            k_marker: PhantomData<K>,
            v_marker: PhantomData<V>,
        }
        impl<K: Deserialize + Hash + Eq, V: Deserialize> Visitor for HashMapVisitor<K, V> {
            type Value = HashMap<K, V>;

            fn expected_a(self) -> String {
                "hashmap".to_string()
            }

            fn visit_map<M: MapAccess>(self, mut map: M) -> Result<Self::Value, AnyError> {
                let mut tmp = HashMap::new();

                while let Some(key) = map.next_key()? {
                    let value = map.next_value()?;
                    tmp.insert(key, value);
                }

                Ok(tmp)
            }
        }

        des.deserialize_map(HashMapVisitor {
            k_marker: PhantomData,
            v_marker: PhantomData,
        })
    }
}

impl_deserialize_primitive!(i8, visit_i8, deserialize_i8);
impl_deserialize_primitive!(i16, visit_i16, deserialize_i16);
impl_deserialize_primitive!(i32, visit_i32, deserialize_i32);
impl_deserialize_primitive!(i64, visit_i64, deserialize_i64);
impl_deserialize_primitive!(i128, visit_i128, deserialize_i128);
impl_deserialize_primitive!(u8, visit_u8, deserialize_u8);
impl_deserialize_primitive!(u16, visit_u16, deserialize_u16);
impl_deserialize_primitive!(u32, visit_u32, deserialize_u32);
impl_deserialize_primitive!(u64, visit_u64, deserialize_u64);
impl_deserialize_primitive!(u128, visit_u128, deserialize_u128);
impl_deserialize_primitive!(f32, visit_f32, deserialize_f32);
impl_deserialize_primitive!(f64, visit_f64, deserialize_f64);
impl_deserialize_primitive!(isize, visit_isize, deserialize_isize);
impl_deserialize_primitive!(usize, visit_usize, deserialize_usize);
