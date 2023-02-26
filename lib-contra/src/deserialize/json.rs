use std::{io::Cursor, str::{from_utf8, FromStr}};

use crate::{error::AnyError};

use super::{MapAccess, Deserialize, Deserializer, Visitor, Peek, SeqAccess};

pub trait FromJson: Sized {
    fn from_json(str: &str) -> Result<Self, AnyError>;
}

impl<D: Deserialize> FromJson for D {
    fn from_json(str: &str) -> Result<Self, AnyError> { 
       let mut de = JsonDeserializer {
           read: Cursor::new(str)
       };
       Self::deserialize(&mut de)
    }
}

macro_rules! impl_deserializer_primitive {
    ($ttype: ident, $deserialize_fn: ident, $parse_fn: ident, $visit_fn: ident) => {
        fn $deserialize_fn<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
            self.parse_whitespaces()?;
            match self.read.peek()? {
                Some(b'-') | Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5') | Some(b'6') | Some(b'7') | Some(b'8') | Some(b'9') | Some(b'.') => {
                    let str = self.read.read_until(&[b' ',b',',b'\t',b'\n',b']',b'}',b':'])?;
                    let str = from_utf8(str.as_slice())?;
                    let val = self.$parse_fn(str)?;
                    v.$visit_fn(val)
                },
                Some(b'"') => {
                    self.read.consume()?;
                    let str = self.read.read_until(&[b'"'])?;
                    self.read.consume()?;
                    let str = from_utf8(str.as_slice())?;
                    let val = self.$parse_fn(str)?;
                    v.$visit_fn(val)
                },
                Some(_) | None => Err(concat!("expected a ", stringify!($ttype), " to start").into()),
            }
        }
    };
    ($ttype: ident as $cast: ident, $deserialize_fn: ident, $parse_fn: ident, $visit_fn: ident) => {
        fn $deserialize_fn<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
            self.parse_whitespaces()?;
            match self.read.peek()? {
                Some(b'-') | Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5') | Some(b'6') | Some(b'7') | Some(b'8') | Some(b'9') | Some(b'.') => {
                    let str = self.read.read_until(&[b' ',b',',b'\t',b'\n',b']',b'}',b':'])?;
                    let str = from_utf8(str.as_slice())?;
                    let val = self.$parse_fn(str)?;
                    v.$visit_fn(val as $cast)
                },
                Some(b'"') => {
                    self.read.consume()?;
                    let str = self.read.read_until(&[b'"'])?;
                    self.read.consume()?;
                    let str = from_utf8(str.as_slice())?;
                    let val = self.$parse_fn(str)?;
                    v.$visit_fn(val)
                },
                Some(_) | None => Err(concat!("expected a ", stringify!($ttype), " to start").into()),
            }
        }
    };
}

pub struct JsonDeserializer<P: Peek> {
    read: P
}

struct JsonMap<'de, P: Peek> {
    de: &'de mut JsonDeserializer<P> 
}

struct JsonArray<'de, P: Peek> {
    de: &'de mut JsonDeserializer<P> 
}

impl<P: Peek> JsonDeserializer<P> {
    pub fn new(peek: P) -> Self {
        Self {
            read: peek,
        }
    }

    fn parse_whitespaces(&mut self) -> Result<(), AnyError> {
        self.read.consume_matching(&[b' ', b'\n', b'\t'])
    }

    fn parse_signed_number<I: FromStr>(&mut self, str: &str) -> Result<I, AnyError>
    where
        <I as FromStr>::Err : std::error::Error + 'static
    {
        str.parse().map_err(|err| Box::from(err))
    }

    fn parse_unsigned_number<U: FromStr>(&mut self, str: &str) -> Result<U, AnyError>
    where
        <U as FromStr>::Err : std::error::Error + 'static
    {
        str.parse().map_err(|err| Box::from(err))
    }

    fn parse_floating_number<F: FromStr>(&mut self, str: &str) -> Result<F, AnyError> 
    where
        <F as FromStr>::Err : std::error::Error + 'static
    {
        str.parse().map_err(|err| Box::from(err))
    }
}

impl<P: Peek> Deserializer for &mut JsonDeserializer<P> {
    fn deserialize_map<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
        self.parse_whitespaces()?;
        match self.read.peek()? {
            Some(b'{') => {
                self.read.consume()?;
                let val = v.visit_map(JsonMap { de: self} );
                self.read.consume()?;
                val
            },
            Some(char) => Err(format!("expected a map to start but got \"{}\" instead", char).into()),
            None => Err("expected a map to start".into()),
        }
    }

    fn deserialize_seq<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
        self.parse_whitespaces()?;
        match self.read.peek()? {
            Some(b'[') => {
                self.read.consume()?;
                v.visit_seq(JsonArray { de: self} )
            },
            Some(_) | None => Err("expected a vec to start".into()),
        }
    }

    fn deserialize_struct<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
        self.deserialize_map(v)
    }
    
    fn deserialize_str<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
        self.parse_whitespaces()?;
        match self.read.peek()? {
            Some(b'"') => {
                self.read.consume()?;
                let str = self.read.read_until(&[b'"'])?;
                self.read.consume()?;
                let str = from_utf8(str.as_slice())?;
                v.visit_str(str)
            },
            Some(_) | None => Err("expected a str to start".into()),
        }
    }

    impl_deserializer_primitive!(i8    , deserialize_i8  , parse_signed_number,    visit_i8  );
    impl_deserializer_primitive!(i16   , deserialize_i16 , parse_signed_number,    visit_i16 );
    impl_deserializer_primitive!(i32   , deserialize_i32 , parse_signed_number,    visit_i32 );
    impl_deserializer_primitive!(i64   , deserialize_i64 , parse_signed_number,    visit_i64 );
    impl_deserializer_primitive!(i128  , deserialize_i128, parse_signed_number,    visit_i128);
    impl_deserializer_primitive!(u8    , deserialize_u8  , parse_unsigned_number,  visit_u8  );
    impl_deserializer_primitive!(u16   , deserialize_u16 , parse_unsigned_number,  visit_u16 );
    impl_deserializer_primitive!(u32   , deserialize_u32 , parse_unsigned_number,  visit_u32 );
    impl_deserializer_primitive!(u64   , deserialize_u64 , parse_unsigned_number,  visit_u64 );
    impl_deserializer_primitive!(u128  , deserialize_u128, parse_unsigned_number,  visit_u128);
    impl_deserializer_primitive!(f32   , deserialize_f32 , parse_floating_number,  visit_f32 );
    impl_deserializer_primitive!(f64   , deserialize_f64 , parse_floating_number,  visit_f64 );
    impl_deserializer_primitive!(isize , deserialize_isize, parse_signed_number  , visit_isize);
    impl_deserializer_primitive!(usize , deserialize_usize, parse_unsigned_number, visit_usize);
}

impl<'de, P: Peek> MapAccess for JsonMap<'de, P> {
    fn next_value<V: Deserialize>(&mut self) -> Result<V, AnyError> {
        self.de.parse_whitespaces()?;
        match self.de.read.peek()? {
             Some(b':') => {
                self.de.read.consume()?;
                self.de.parse_whitespaces()?;
                match self.de.read.peek()? {
                    Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5') | Some(b'6') | Some(b'7') | Some(b'8') | Some(b'9') | Some(b'-') | Some(b'"') | Some(b'{') | Some(b'[') => Ok(V::deserialize(&mut *self.de)?),
                    Some(_) | None => Err("expected a map value".into()),
                }
             },
             Some(_) | None => Err("expected a map assignment".into()),
        }
    }

    fn next_key<K: Deserialize>(&mut self) -> Result<Option<K>, AnyError> {
        self.de.parse_whitespaces()?;
        match self.de.read.peek()? {
            Some(b'"') => Ok(Some(K::deserialize(&mut *self.de)?)),
            Some(b',') => { 
                self.de.read.consume()?;
                self.next_key()
            },
            Some(b'}') => Ok(None),
            Some(_) | None => Err("expected a map key".into()),
        }
    }
}

impl<'de, P: Peek> SeqAccess for JsonArray<'de, P> {
    fn next_value<V: Deserialize>(&mut self) -> Result<Option<V>, AnyError> {
        self.de.parse_whitespaces()?;
        match self.de.read.peek()? {
            Some(b',') => { self.de.read.consume()?; self.next_value() }
            Some(b']') => { self.de.read.consume()?; Ok(None) }
            Some(b'"') | Some(_) => Ok(Some(V::deserialize(&mut *self.de)?)),
            None => Err("expected a seq element".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{io::Cursor, collections::HashMap};

    use super::*;

    #[test]
    fn parse_vec_test() {
        let expected = vec![32i32, 64i32];

        let input = "[32, 64]";
        let input = Cursor::new(input);
        let mut de = JsonDeserializer { read: input };
        let result = Vec::<i32>::deserialize(&mut de);

        dbg!(&result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn parse_map_test() {
        let mut expected = HashMap::new();
        expected.insert(2i32, 32i32);

        let input = "{ \"2\": 32 }";
        let input = Cursor::new(input);
        let mut de = JsonDeserializer { read: input };
        let map = HashMap::<i32, i32>::deserialize(&mut de);

        dbg!(&map);
        assert!(map.is_ok());
        assert_eq!(map.unwrap(), expected);
    }

    #[test]
    fn parse_struct_test() {
        #[derive(Debug)]
        struct A {
            a: i32,
            s: String
        }

        impl Deserialize for A {
            fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError> {
                enum Field {
                    A,
                    S
                }

                impl Deserialize for Field {
                    fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError> {
                        struct FieldVisitor {}
                        impl Visitor for FieldVisitor {
                            type Value = Field;

                            fn visit_str(self, v: &str) -> Result<Self::Value, AnyError> {
                                match v {
                                    "a" => Ok(Field::A),
                                    "s" => Ok(Field::S),
                                    val => Err(format!("unknown \"{}\" field for A", val).into())
                                }
                            }

                            fn expected_a(self) -> String {
                                "A field".into()
                            }
                        }

                        des.deserialize_str(FieldVisitor {})
                    }
                }

                struct AVisitor {}
                impl Visitor for AVisitor {
                    type Value = A;

                    fn expected_a(self) -> String {
                        "A".into()
                    }

                    fn visit_map<M: MapAccess>(self, mut map: M) -> Result<Self::Value, AnyError> {
                        let mut a = None;
                        let mut s = None;

                        while let Some(key) = map.next_key()? {
                            match key {
                                Field::A => { 
                                    if a.is_some() {
                                        return Err("duplicate field a".into());
                                    };
                                    a = Some(map.next_value()?) 
                                },
                                Field::S => {
                                    if s.is_some() {
                                        return Err("duplicate field s".into());
                                    };
                                    s = Some(map.next_value()?) 
                                }
                            }
                        }

                        let a = a.ok_or_else(|| "missing field a")?;
                        let s = s.ok_or_else(|| "missing field s")?;

                        Ok(A {
                            a: a,
                            s: s
                        })
                    }
                }

                des.deserialize_struct(AVisitor {})
            }
        }

        let input = "{ \"a\": \"32\", \"s\": \"well well well\" }";
        let input = Cursor::new(input);

        let mut de = JsonDeserializer { read: input };
        
        let a = A::deserialize(&mut de);

        dbg!(&a);
        assert!(a.is_ok());
        assert_eq!(a.as_ref().unwrap().a, 32);
        assert_eq!(a.as_ref().unwrap().s, "well well well".to_string());
    }
}