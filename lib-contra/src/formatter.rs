use std::io;

use crate::error::{IoResult};

macro_rules! decl_write_primitive {
    ($type: ident, $ser_func: ident) => {
        fn $ser_func(&mut self, write: &mut W, value: &$type) -> IoResult;
    };
}

pub trait Formatter<W: io::Write> {
    fn write_struct_begin(&mut self, write: &mut W, name: &str, fields: usize) -> IoResult;
    fn write_struct_end(&mut self, write: &mut W, name: &str) -> IoResult;

    fn write_field_assignnment_begin(&mut self, write: &mut W) -> IoResult;
    fn write_field_key(&mut self, write: &mut W, name: &str) -> IoResult;
    fn write_field_assignnment_operator(&mut self, write: &mut W) -> IoResult;
    fn write_field_assignnment_end(&mut self, write: &mut W) -> IoResult;

    decl_write_primitive!(i32, write_i32);
}