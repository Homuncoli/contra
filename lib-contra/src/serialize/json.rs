use std::io;
use std::mem::size_of;

use crate::error::{IoResult, AnyError};
use crate::formatter::Formatter;

use crate::serialize::Serialize;
use crate::serializer::{Serializer};

pub trait IntoJson{
    fn to_json(&self) -> Result<String, AnyError>;
}

impl<S: Serialize> IntoJson for S {
    fn to_json(&self) -> Result<String, AnyError> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<S>());
        let formatter = PrettyJsonFormatter::new("\t".to_string());
        let mut serializer = JsonSerializer::new(formatter, &mut buffer);

        self.serialize(&mut serializer)?;
        
        unsafe { Ok(String::from_utf8_unchecked(buffer)) }
    }
}

pub struct JsonSerializer<'w, W: io::Write, F: Formatter<W>> {
    write: &'w mut W,
    formatter: F,
}

impl<'w, W: io::Write, F: Formatter<W>> JsonSerializer<'w, W, F> {
    pub fn new(formatter: F, write: &'w mut W) -> Self {
        Self {
            formatter,
            write
        }
    }
}

impl<'w, W: io::Write, F: Formatter<W>> Serializer for JsonSerializer<'w, W, F> {
    fn begin_struct(&mut self, name: &str, fields: usize) -> crate::error::SuccessResult {
        self.formatter.write_struct_begin(self.write, name, fields)?;
        Ok(())
    }

    fn end_struct(&mut self, name: &str)-> crate::error::SuccessResult {
        self.formatter.write_struct_end(self.write, name)?;
        Ok(())
    }

    fn serialize_field<V: crate::serialize::Serialize>(&mut self, identifier: &str, value: &V)-> crate::error::SuccessResult {
        self.formatter.write_field_assignnment_begin(self.write)?;
        self.formatter.write_field_key(self.write, identifier)?;
        self.formatter.write_field_assignnment_operator(self.write)?;
        value.serialize(self)?;
        self.formatter.write_field_assignnment_end(self.write)?;
        Ok(())
    }

    fn serialize_value<V: crate::serialize::Serialize>(&mut self, value: &V) -> crate::error::SuccessResult {
        value.serialize(self)?;
        Ok(())
    }

    fn serialize_i32(&mut self, value: &i32) -> crate::error::SuccessResult {
        self.formatter.write_i32(self.write, value)?;
        Ok(())
    }
}

pub struct PrettyJsonFormatter {
    ident_sym: String,
    ident_num: usize,
}

impl PrettyJsonFormatter {
    fn new(sym: String) -> Self {
        Self {
            ident_sym: sym,
            ident_num: 0,
        }
    }

    fn increase_ident(&mut self) {
        self.ident_num += 1;
    }

    fn decrease_ident(&mut self) {
        self.ident_num -= 1;
    }

    fn write_ident<W: io::Write>(&mut self, write: &mut W) -> IoResult {
        write.write_all(self.ident_sym.repeat(self.ident_num).as_bytes())?;
        Ok(())
    }

    fn write_unescaped_string<W: io::Write>(&mut self, write: &mut W, value: &str) -> IoResult {
        write.write_all(value.as_bytes())?;
        Ok(())
    }

    fn write_escaped_string<W: io::Write>(&mut self, write: &mut W, value: &str) -> IoResult {
        write.write_fmt(format_args!("\"{}\"", value))?;
        Ok(())
    }

    fn write_line_break<W: io::Write>(&mut self, write: &mut W) -> IoResult {
        // ToDo: Make linebreak platform independant
        write.write_all(b"\n")?;
        Ok(())
    }

    fn write_whitespace<W: io::Write>(&mut self, write: &mut W, n: usize) -> IoResult {
        write.write_all(" ".repeat(n).as_bytes())?;
        Ok(())
    }

    fn write_seperator<W: io::Write>(&mut self, write: &mut W) -> IoResult {
        write.write_all(b",")?;
        Ok(())
    }
}

macro_rules! impl_write_json_primitve {
    ($type: ident, $ser_func: ident) => {
        fn $ser_func(&mut self, write: &mut W, value: &$type) -> IoResult {
            self.write_escaped_string(write, &value.to_string())?;
            Ok(())
        }
    };
}

impl<W: io::Write> Formatter<W> for PrettyJsonFormatter {
    fn write_struct_begin(&mut self, write: &mut W, _name: &str, _fields: usize) -> IoResult {
        self.write_ident(write)?;
        self.write_unescaped_string(write, "{")?;
        self.write_line_break(write)?;
        self.increase_ident();
        Ok(())
    }

    fn write_struct_end(&mut self, write: &mut W, _name: &str) -> IoResult {
        self.decrease_ident();
        self.write_ident(write)?;
        self.write_unescaped_string(write, "}")?;
        self.write_line_break(write)?;
        Ok(())
    }

    fn write_field_assignnment_begin(&mut self, write: &mut W) -> IoResult {
        self.write_ident(write)?;
        Ok(())
    }

    fn write_field_key(&mut self, write: &mut W, name: &str) -> IoResult {
        self.write_escaped_string(write, name)?;
        self.write_whitespace(write, 1)?;
        Ok(())
    }

    fn write_field_assignnment_operator(&mut self, write: &mut W) -> IoResult {
        self.write_unescaped_string(write, ":")?;
        self.write_whitespace(write, 1)?;
        Ok(())
    }

    fn write_field_assignnment_end(&mut self, write: &mut W) -> IoResult {
        self.write_seperator(write)?;
        self.write_line_break(write)?;
        Ok(())
    }

    impl_write_json_primitve!(i32, write_i32);
}

#[cfg(test)]
mod test {
    use crate::{serialize::{Serialize, json::IntoJson}, serializer::Serializer, error::SuccessResult};

    struct PrimitiveDataTypesStruct {
        i32: i32,
    }

    impl PrimitiveDataTypesStruct {
        fn new() -> Self {
            PrimitiveDataTypesStruct { 
                i32: i32::MAX,
             }
        }
    }

    impl Serialize for PrimitiveDataTypesStruct {
        fn serialize<S: Serializer>(&self, ser: &mut S) -> SuccessResult {
            ser.begin_struct("PrimitiveDataTypesStruct", 1)?;

            ser.serialize_field("i32", &self.i32)?;

            ser.end_struct("PrimitiveDataTypesStruct")?;

            Ok(())
        }
    }

    #[test]
    fn serializing_struct_works() {
        let obj = PrimitiveDataTypesStruct::new();

        let json = IntoJson::to_json(&obj);

        assert!(json.is_ok());
        println!("{}", json.unwrap());
    }
}