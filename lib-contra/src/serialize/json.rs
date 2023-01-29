use std::io;
use std::mem::size_of;

use crate::error::{IoResult, AnyError, SuccessResult};
use crate::formatter::WriteFormatter;

use crate::position::Position;
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

        self.serialize(&mut serializer, &Position::Closing)?;
        
        unsafe { Ok(String::from_utf8_unchecked(buffer)) }
    }
}

pub struct JsonSerializer<'w, W: io::Write, F: WriteFormatter<W>> {
    write: &'w mut W,
    formatter: F,
}

impl<'w, W: io::Write, F: WriteFormatter<W>> JsonSerializer<'w, W, F> {
    pub fn new(formatter: F, write: &'w mut W) -> Self {
        Self {
            formatter,
            write
        }
    }
}


macro_rules! impl_serialize_primitive {
    ($type: ident, $ser_func: ident, $for_func: ident) => {
        fn $ser_func(&mut self, value: &$type) -> SuccessResult {
            self.formatter.$for_func(self.write, value)?;
            Ok(())
        }
    };
}

impl<'w, W: io::Write, F: WriteFormatter<W>> Serializer for JsonSerializer<'w, W, F> {
    fn begin_struct(&mut self, name: &str, fields: usize) -> SuccessResult {
        self.formatter.write_struct_begin(self.write, name, fields)?;
        Ok(())
    }

    fn end_struct(&mut self, name: &str) -> SuccessResult {
        self.formatter.write_struct_end(self.write, name)?;
        Ok(())
    }

    fn serialize_field<V: crate::serialize::Serialize>(&mut self, identifier: &str, value: &V, pos: &Position)-> SuccessResult {
        self.formatter.write_field_assignnment_begin(self.write)?;
        self.formatter.write_field_key(self.write, identifier)?;
        self.formatter.write_field_assignnment_operator(self.write)?;
        value.serialize(self, &pos)?;
        self.formatter.write_field_assignnment_end(self.write, &pos)?;
        Ok(())
    }

    fn serialize_value<V: crate::serialize::Serialize>(&mut self, value: &V, pos: &Position) -> SuccessResult {
        value.serialize(self, pos)?;
        Ok(())
    }

    impl_serialize_primitive!(i8,    serialize_i8,      write_i8);
    impl_serialize_primitive!(i16,   serialize_i16,     write_i16);
    impl_serialize_primitive!(i32,   serialize_i32,     write_i32);
    impl_serialize_primitive!(i64,   serialize_i64,     write_i64);
    impl_serialize_primitive!(i128,  serialize_i128,    write_i128);
    impl_serialize_primitive!(u8  ,  serialize_u8,      write_u8);
    impl_serialize_primitive!(u16 ,  serialize_u16,     write_u16);
    impl_serialize_primitive!(u32 ,  serialize_u32,     write_u32);
    impl_serialize_primitive!(u64 ,  serialize_u64,     write_u64);
    impl_serialize_primitive!(u128,  serialize_u128,    write_u128);
    impl_serialize_primitive!(usize, serialize_usize,   write_usize);
    impl_serialize_primitive!(isize, serialize_isize,   write_isize);
    impl_serialize_primitive!(String,serialize_string,  write_string);
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

macro_rules! impl_write_primitive {
    ($type: ident, $ser_func: ident) => {
        fn $ser_func(&mut self, write: &mut W, value: &$type) -> IoResult {
            self.write_escaped_string(write, &value.to_string())?;
            Ok(())
        }
    };
}

impl<W: io::Write> WriteFormatter<W> for PrettyJsonFormatter {
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

    fn write_field_assignnment_end(&mut self, write: &mut W, pos: &Position) -> IoResult {
        match pos {
            Position::Trailing => self.write_seperator(write)?,
            Position::Closing => (),
        }
        self.write_line_break(write)?;
        Ok(())
    }

    impl_write_primitive!(i8,    write_i8);
    impl_write_primitive!(i16,   write_i16);
    impl_write_primitive!(i32,   write_i32);
    impl_write_primitive!(i64,   write_i64);
    impl_write_primitive!(i128,  write_i128);
    impl_write_primitive!(u8  ,  write_u8);
    impl_write_primitive!(u16 ,  write_u16);
    impl_write_primitive!(u32 ,  write_u32);
    impl_write_primitive!(u64 ,  write_u64);
    impl_write_primitive!(u128,  write_u128);
    impl_write_primitive!(usize, write_usize);
    impl_write_primitive!(isize, write_isize);
    impl_write_primitive!(String,write_string);
}
