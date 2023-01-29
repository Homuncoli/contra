use std::{io::{self, Cursor, SeekFrom}, str::from_utf8};

use crate::{error::{AnyError, SuccessResult}, deserializer::Deserializer, formatter::ReadFormatter};

use super::Deserialize;

pub trait FromJson: Sized {
    fn from_json(json: &str) -> Result<Self, AnyError>;
}

impl<T: Deserialize> FromJson for T {
    fn from_json(json: &str) -> Result<Self, AnyError> {
        let mut cursor = Cursor::new(json.as_bytes());
        let mut des = JsonDeserializer::new(&mut cursor);
        Self::deserialize(&mut des)
    }
}

pub struct JsonFormatter {}

impl JsonFormatter {
    fn new() -> Self {
        Self {

        }
    }

    // if the next bytes match **expected**, they remain consumed, otherwise its rewund
    fn continues_with_unescaped<R: io::Read + io::Seek>(&mut self, read: &mut R, expected: &[u8]) -> Result<bool, AnyError> {
        let prev_pos = read.stream_position()?;

        let mut buffer: Vec<u8> = vec![0; expected.len()];
        read.read_exact(&mut buffer)?;

        if buffer == expected {
            Ok(true)
        } else {
            read.seek(SeekFrom::Start(prev_pos))?;
            Ok(false)
        }
    }

    // if the next bytes match "**expected**"", they remain consumed, otherwise its rewund
    fn continues_with_escaped<R: io::Read + io::Seek>(&mut self, read: &mut R, expected: &[u8]) -> Result<bool, AnyError> {
        let expected = [b"\"", expected, b"\""].concat();
        self.continues_with_unescaped(read, &expected)
    }

    fn read_escaped_string<R: io::Read + io::Seek>(&mut self, read: &mut R) -> Result<String, AnyError> {
        let prev_pos = read.stream_position()?;

        if !self.continues_with_unescaped(read, b"\"")? {
            read.seek(SeekFrom::Start(prev_pos))?;
            return Err("expected an escaped string".into());
        }

        let result = self.read_until(read, b"\"");
        if result.is_err() {
            read.seek(SeekFrom::Start(prev_pos))?;
        }
        
        let binding = result.unwrap();
        let mut output = binding.chars();
        output.next_back();
        Ok(output.as_str().to_string())
    }

    fn read_until<R: io::Read + io::Seek>(&mut self, read: &mut R, stop: &[u8]) -> Result<String, AnyError> {
        let start_pos = read.stream_position()?;
        let mut output: String = "".to_string();
        let mut buffer: [u8; 1] = [0; 1];

        while buffer != stop {
            let read_result = read.read_exact(&mut buffer);
            if read_result.is_err() {
                read.seek(SeekFrom::Start(start_pos))?;
            }

            let parse_result = from_utf8(&buffer);
            if read_result.is_err() {
                read.seek(SeekFrom::Start(start_pos))?;
            }

            output += parse_result.unwrap();
        }
        Ok(output)
    }

    // consumes the next idents and linebreaks
    fn strip<R: io::Read + io::Seek>(&mut self, read: &mut R) -> SuccessResult {
        let mut result = true;

        while result {    
            result = false;
            result |= self.continues_with_unescaped(read, b" ")?;
            if result { continue; }
            result |= self.continues_with_unescaped(read, b"\t")?;
            if result { continue; }
            //ToDo: make linebreaks platform independant
            result |= self.continues_with_unescaped(read, b"\n")?;
        }

        Ok(())
    }
}

impl<R: io::Read + io::Seek> ReadFormatter<R> for JsonFormatter {
    fn read_struct_begin(&mut self, read: &mut R, _name: &str, _fields: usize) -> SuccessResult {
        self.strip(read)?;
        let valid = self.continues_with_unescaped(read, b"{")?;
        if valid {
            Ok(())
        } else {
            Err("expected a struct begin \"{\"".into())
        }
    }

    fn read_struct_end(&mut self, read: &mut R, _name: &str) -> SuccessResult {
        self.strip(read)?;
        let valid = self.continues_with_unescaped(read, b"}")?;
        if valid {
            Ok(())
        } else {
            Err("expected a struct end \"}\"".into())
        }
    }

    fn read_field_assignnment_begin(&mut self, _read: &mut R) -> SuccessResult {
        Ok(())
    }

    fn read_field_key(&mut self, read: &mut R, name: &str) -> SuccessResult {
        self.strip(read)?;
        let valid = self.continues_with_escaped(read, name.as_bytes())?;
        if valid {
            Ok(())
        } else {
            Err(format!("expected field \"{}\"", name).into())
        }
    }

    fn read_field_assignnment_operator(&mut self, read: &mut R) -> SuccessResult {
        self.strip(read)?;
        let valid = self.continues_with_unescaped(read, b":")?;
        if valid {
            Ok(())
        } else {
            Err(format!("expected assignement operator \"{}\"", ":").into())
        }
    }

    fn read_field_assignnment_end(&mut self, read: &mut R) -> SuccessResult {
        self.strip(read)?;
        let valid = self.continues_with_unescaped(read, b",")?;
        if valid {
            Ok(())
        } else {
            println!("{}", "WARN: no assignement end");
            Ok(())
        }
    }

    fn read_i32(&mut self, read: &mut R) -> Result<i32,AnyError> {
        self.strip(read)?;
        let literal = self.read_escaped_string(read)?;
        let value = literal.parse::<i32>()?;
        Ok(value)
    }
}

pub struct JsonDeserializer<'r, R: io::Read + io::Seek> {
    read: &'r mut R,
    formatter: JsonFormatter,
}

impl<'r, R: io::Read + io::Seek> JsonDeserializer<'r, R> {
    pub fn new(json: &'r mut R) -> Self {
        Self {
            read: json,
            formatter: JsonFormatter::new()
        }
    }
}

impl<'r, R: io::Read + io::Seek> Deserializer for JsonDeserializer<'r, R> {
    fn deserialize_struct_begin(&mut self, name: &str, fields: usize) -> SuccessResult {
        self.formatter.read_struct_begin(self.read, name, fields)?;
        Ok(())
    }

    fn deserialize_struct_end(&mut self, name: &str) -> SuccessResult {
        self.formatter.read_struct_end(self.read, name)?;
        Ok(())
    }

    fn deserialize_field<T: Deserialize>(&mut self, field: &str) -> Result<T, AnyError> {
        self.formatter.read_field_assignnment_begin(&mut self.read)?;
        self.formatter.read_field_key(&mut self.read, field)?;
        self.formatter.read_field_assignnment_operator(&mut self.read)?;
        let value = T::deserialize(self)?;
        self.formatter.read_field_assignnment_end(&mut self.read)?;
        Ok(value)
    }

    fn deserialize_i32(&mut self) -> Result<i32, AnyError> {
        self.formatter.read_i32(&mut self.read)
    }
}

#[cfg(test)]
mod test {
    use crate::{error::{AnyError}, deserialize::Deserialize, deserializer::{Deserializer}};

    use super::FromJson;

    #[derive(Debug)]
    struct PrimitiveDataTypesStruct {
        i32: i32,
    }

    impl Deserialize for PrimitiveDataTypesStruct {
        fn deserialize<D: Deserializer>(des: &mut D) -> Result<Self, crate::error::AnyError> {
            des.deserialize_struct_begin("PrimitiveDataTypesStruct", 1)?;

            let i32 = des.deserialize_field("i32")?;

            des.deserialize_struct_end("PrimitiveDataTypesStruct")?;

            let obj = Self {
                i32,
            };

            Ok(obj)
        }
    }

    #[test]
    fn deserializing_struct_works() {
        let json = "
        {
            \"i32\":\"2147483647\"
        }
        ";

        let result: Result<PrimitiveDataTypesStruct, AnyError> = FromJson::from_json(json);

        dbg!(&result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().i32, i32::MAX);
    }
}