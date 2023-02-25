use std::{io::Cursor, str::from_utf8};

use crate::{error::AnyError};

use super::{MapAccess, Deserialize, Deserializer, Visitor, Peek};

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

pub struct JsonDeserializer<P: Peek> {
    read: P
}

pub struct JsonParser {}

struct JsonMap<'de, P: Peek> {
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

    fn parse_signed_number(&mut self, str: &str) -> Result<i32, AnyError> {
        str.parse().map_err(|err| <Box<dyn std::error::Error>>::from(err))
    }

    fn parse_unsigned_number(&mut self, str: &str) -> Result<u32, AnyError> {
        str.parse().map_err(|err| <Box<dyn std::error::Error>>::from(err))
    }

    fn parse_floating_number(&mut self, str: &str) -> Result<f32, AnyError> {
        str.parse().map_err(|err| <Box<dyn std::error::Error>>::from(err))
    }
}

impl<P: Peek> Deserializer for &mut JsonDeserializer<P> {
    fn deserialize_map<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
        self.parse_whitespaces()?;
        match self.read.peek()? {
            Some(b'{') => {
                self.read.consume()?;
                v.visit_map(JsonMap { de: self} )
            },
            Some(_) | None => Err("expected a map to start".into()),
        }
    }

    fn deserialize_struct<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
        self.deserialize_map(v)
    }
    
    fn deserialize_i32<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
        self.parse_whitespaces()?;
        match self.read.peek()? {
            Some(b'"') => {
                self.read.consume()?;
                let str = self.read.read_until(b'"')?;
                self.read.consume()?;
                let str = from_utf8(str.as_slice())?;
                let val = self.parse_signed_number(str)?;
                v.visit_i32(val)
            },
            Some(_) | None => Err("expected a str to start".into()),
        }
    }

    fn deserialize_str<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> {
        self.parse_whitespaces()?;
        match self.read.peek()? {
            Some(b'"') => {
                self.read.consume()?;
                let str = self.read.read_until(b'"')?;
                self.read.consume()?;
                let str = from_utf8(str.as_slice())?;
                v.visit_str(str)
            },
            Some(_) | None => Err("expected a str to start".into()),
        }
    }
}

impl<'de, P: Peek> MapAccess for JsonMap<'de, P> {
    fn next_value<V: Deserialize>(&mut self) -> Result<V, AnyError> {
        self.de.parse_whitespaces()?;
        match self.de.read.peek()? {
             Some(b':') => {
                self.de.read.consume()?;
                self.de.parse_whitespaces()?;
                match self.de.read.peek()? {
                    Some(b'"') => Ok(V::deserialize(&mut *self.de)?),
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
             Some(b'}') => Ok(None),
             Some(_) | None => Err("expected a map key".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test() {
        #[derive(Debug)]
        struct A {
            a: i32,
        }

        impl Deserialize for A {
            fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError> {
                enum Field {
                    a
                }

                impl Deserialize for Field {
                    fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError> {
                        struct FieldVisitor {}
                        impl Visitor for FieldVisitor {
                            type Value = Field;

                            fn visit_str(self, v: &str) -> Result<Self::Value, AnyError> {
                                match v {
                                    "a" => Ok(Field::a),
                                    val => Err(format!("unknown \"{}\" field for A", val).into())
                                }
                            }

                            fn expected_a(self) -> String {
                                "A field".into()
                            }
                        }

                        des.deserialize_identifier(FieldVisitor {})
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

                        while let Some(key) = map.next_key()? {
                            match key {
                                Field::a => { 
                                    if a.is_some() {
                                        return Err("duplicate field a".into());
                                    };
                                    a = Some(map.next_value()?) 
                                }
                            }
                        }

                        let a = a.ok_or_else(|| "missing field a")?;

                        Ok(A {
                            a: a
                        })
                    }
                }

                des.deserialize_struct(AVisitor {})
            }
        }

        let input = "{ \"a\": \"32\" }";
        let input = Cursor::new(input);

        let mut de = JsonDeserializer { read: input };
        
        let a = A::deserialize(&mut de);

        dbg!(&a);
        assert!(a.is_ok());
        assert_eq!(a.unwrap().a, 32);
    }
}