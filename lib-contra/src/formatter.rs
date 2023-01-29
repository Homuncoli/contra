use std::io;

use crate::{error::{IoResult, SuccessResult, AnyError}, position::Position};

macro_rules! decl_write_primitive {
    ($type: ident, $ser_func: ident) => {
        fn $ser_func(&mut self, write: &mut W, value: &$type) -> IoResult;
    };
}

pub trait WriteFormatter<W: io::Write> {
    fn write_struct_begin(&mut self, write: &mut W, name: &str, fields: usize) -> IoResult;
    fn write_struct_end(&mut self, write: &mut W, name: &str) -> IoResult;

    fn write_collection_begin(&mut self, write: &mut W, name: &str, size: usize) -> IoResult;
    fn write_collection_end(&mut self, write: &mut W, name: &str)-> IoResult;

    fn write_field_assignnment_begin(&mut self, write: &mut W) -> IoResult;
    fn write_field_key(&mut self, write: &mut W, name: &str) -> IoResult;
    fn write_field_assignnment_operator(&mut self, write: &mut W) -> IoResult;
    fn write_field_assignnment_end(&mut self, write: &mut W, pos: &Position) -> IoResult;

    decl_write_primitive!(i8,    write_i8);
    decl_write_primitive!(i16,   write_i16);
    decl_write_primitive!(i32,   write_i32);
    decl_write_primitive!(i64,   write_i64);
    decl_write_primitive!(i128,  write_i128);
    decl_write_primitive!(u8  ,  write_u8);
    decl_write_primitive!(u16 ,  write_u16);
    decl_write_primitive!(u32 ,  write_u32);
    decl_write_primitive!(u64 ,  write_u64);
    decl_write_primitive!(u128,  write_u128);
    decl_write_primitive!(usize, write_usize);
    decl_write_primitive!(isize, write_isize);
    decl_write_primitive!(str,   write_str);
}


macro_rules! decl_read_primitive {
    ($ttype: ident, $des_func: ident) => {
        fn $des_func(&mut self, read: &mut R) -> Result<$ttype, AnyError>;
    };
}

pub trait ReadFormatter<R: io::Read + io::Seek> {
    fn read_struct_begin(&mut self, read: &mut R, name: &str, fields: usize) -> SuccessResult;
    fn read_struct_end(&mut self, read: &mut R, name: &str) -> SuccessResult;

    fn read_vec_begin(&mut self, read: &mut R, name: &str) -> SuccessResult;
    fn read_vec_end(&mut self, read: &mut R, name: &str) -> SuccessResult;

    fn read_field_assignnment_begin(&mut self, read: &mut R) -> SuccessResult;
    fn read_field_key(&mut self, read: &mut R, name: &str) -> SuccessResult;
    fn read_field_assignnment_operator(&mut self, read: &mut R) -> SuccessResult;
    fn read_field_assignnment_end(&mut self, read: &mut R) -> SuccessResult;

    decl_read_primitive!(i8,    read_i8);
    decl_read_primitive!(i16,   read_i16);
    decl_read_primitive!(i32,   read_i32);
    decl_read_primitive!(i64,   read_i64);
    decl_read_primitive!(i128,  read_i128);
    decl_read_primitive!(u8  ,  read_u8);
    decl_read_primitive!(u16 ,  read_u16);
    decl_read_primitive!(u32 ,  read_u32);
    decl_read_primitive!(u64 ,  read_u64);
    decl_read_primitive!(u128,  read_u128);
    decl_read_primitive!(usize, read_usize);
    decl_read_primitive!(isize, read_isize);
    decl_read_primitive!(String,read_string);
}