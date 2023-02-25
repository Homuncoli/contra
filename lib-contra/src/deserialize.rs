pub mod json;

use std::io::{self, ErrorKind};

use crate::error::AnyError;

use self::error::{visiting_but_expected};

pub trait Deserialize: Sized {
    fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError>;
}

pub trait Deserializer: Sized {
    fn deserialize_map<V: Visitor>(self, v: V) -> Result<V::Value, AnyError>;
    fn deserialize_struct<V: Visitor>(self, v: V) -> Result<V::Value, AnyError>;
    fn deserialize_identifier<V: Visitor>(self, v: V) -> Result<V::Value, AnyError> { self.deserialize_str(v) }
    fn deserialize_str<V: Visitor>(self, v: V) -> Result<V::Value, AnyError>;
    fn deserialize_i32<V: Visitor>(self, v: V) -> Result<V::Value, AnyError>;
}

pub trait Visitor: Sized {
    type Value;
    fn expected_a(self) -> String;

    fn visit_map<M: MapAccess>(self, map: M) -> Result<Self::Value, AnyError> { Err(visiting_but_expected("map", &self.expected_a()).into()) } 
    fn visit_str(self, v: &str) -> Result<Self::Value, AnyError> { Err(visiting_but_expected("str", &self.expected_a()).into()) }
    fn visit_i32(self, v: i32) -> Result<Self::Value, AnyError> { Err(visiting_but_expected("i32", &self.expected_a()).into()) }
}

pub trait MapAccess {
    fn next_value<V: Deserialize>(&mut self) -> Result<V, AnyError> { todo!() }
    fn next_key<K: Deserialize>(&mut self) -> Result<Option<K>, AnyError> { todo!() }
}

mod error {
    pub(crate) fn visiting_but_expected(etype: &str, gtype: &str) -> String {
        "visiting a ".to_string() + etype + " but expected a " + gtype
    }
}

// ##########################################

pub trait Peek {
    fn peek(&mut self) -> Result<Option<u8>, AnyError>;
    fn read_until(&mut self, end: u8) -> Result<Vec<u8>, AnyError>;
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
        char
            .map(|o| { if o.is_some() { return Some(*o.unwrap().get(0).unwrap()) } return None; })
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
                self.consume();
            } else {
                break;
            }
        }
        Ok(())
    }

    fn read_until(&mut self, end: u8) -> Result<Vec<u8>, AnyError> {
        let mut vec = vec![];
        loop {
            let char = self.peek()?;
            if char.is_some() && char.unwrap() != end {
                self.consume();
                vec.push(char.unwrap());
            } else {
                break;
            }
        }
        Ok(vec)
    }
}

// ##########################################

impl Deserialize for i32 {
    fn deserialize<D: Deserializer>(des: D) -> Result<Self, AnyError> {
        struct i32Visitor {}
        impl Visitor for i32Visitor {
            type Value = i32;

            fn expected_a(self) -> String {
                "i32".to_string()
            }

            fn visit_i32(self, v: i32) -> Result<Self::Value, AnyError> {
                Ok(v)
            }
        }

        des.deserialize_i32(i32Visitor {})
    }
}
