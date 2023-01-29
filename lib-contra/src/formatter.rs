use std::io;

use crate::error::{IoResult, SuccessResult, AnyError};

macro_rules! decl_write_primitive {
    ($type: ident, $ser_func: ident) => {
        fn $ser_func(&mut self, write: &mut W, value: &$type) -> IoResult;
    };
}

pub trait WriteFormatter<W: io::Write> {
    fn write_struct_begin(&mut self, write: &mut W, name: &str, fields: usize) -> IoResult;
    fn write_struct_end(&mut self, write: &mut W, name: &str) -> IoResult;

    fn write_field_assignnment_begin(&mut self, write: &mut W) -> IoResult;
    fn write_field_key(&mut self, write: &mut W, name: &str) -> IoResult;
    fn write_field_assignnment_operator(&mut self, write: &mut W) -> IoResult;
    fn write_field_assignnment_end(&mut self, write: &mut W) -> IoResult;

    decl_write_primitive!(i32, write_i32);
}


macro_rules! decl_read_primitive {
    ($ttype: ident, $des_func: ident) => {
        fn $des_func(&mut self, read: &mut R) -> Result<$ttype, AnyError>;
    };
}

pub trait ReadFormatter<R: io::Read + io::Seek> {
    fn read_struct_begin(&mut self, read: &mut R, name: &str, fields: usize) -> SuccessResult;
    fn read_struct_end(&mut self, read: &mut R, name: &str) -> SuccessResult;

    fn read_field_assignnment_begin(&mut self, read: &mut R) -> SuccessResult;
    fn read_field_key(&mut self, read: &mut R, name: &str) -> SuccessResult;
    fn read_field_assignnment_operator(&mut self, read: &mut R) -> SuccessResult;
    fn read_field_assignnment_end(&mut self, read: &mut R) -> SuccessResult;

    decl_read_primitive!(i32, read_i32);
}