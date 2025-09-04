use std::{io::Read, string::FromUtf8Error};

// This mean varint or varlong cant be more than 32-bit or 64-bit val
pub const VARINT_LENGTH: i8 = 5;
pub const VARLONG_LENGTH: i8 = 10;
pub const STRING_LENGTH: usize = 32767;

// -- Aliases --
pub type Boolean = bool;
pub type Byte = i8;
pub type UByte = u8;
pub type Short = i16;
pub type UShort = u16;
pub type Int = i32;
pub type Long = i64;
pub type Float = f32;
pub type Double = f64;
pub type Angle = u8;
pub type UUID = u128;
// -- Aliases end --

// -- types --
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarInt(pub Int);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarLong(pub Long);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedArray<T> {
    pub length: VarInt,
    pub data: Vec<T>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringMC(pub String);
#[derive(Debug)]
// -- types end --

pub enum DecodeError {
    Io(std::io::Error),
    InvalidValue(String),
    Utf8(FromUtf8Error),
}

impl From<std::io::Error> for DecodeError {
    fn from(err: std::io::Error) -> Self {
        DecodeError::Io(err)
    }
}

pub trait Decode: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>;
}

#[derive(Debug)]
pub enum EncodeError {
    Io(std::io::Error),
    ProtocolViolation(String),
}

impl From<FromUtf8Error> for DecodeError {
    fn from(value: FromUtf8Error) -> Self {
        DecodeError::Utf8(value)
    }
}

impl From<std::io::Error> for EncodeError {
    fn from(err: std::io::Error) -> Self {
        EncodeError::Io(err)
    }
}

pub trait Encode: Sized {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError>;
}