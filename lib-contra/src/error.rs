use std::{error::Error, io::{self}};

pub type AnyError = Box<dyn Error>;
pub type SuccessResult = Result<(), AnyError>;
pub type IoResult = Result<(), io::Error>;