use std::io::Read;

use crate::packets::types::types::{Boolean, Decode, DecodeError, Encode, EncodeError};

impl Decode for Boolean {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?; // `?` здесь по-прежнему работает как надо!

        match buffer[0] {
            0x01 => Ok(true),
            0x00 => Ok(false),
            // Любое другое значение...
            other_value => {
                // ...это ошибка невалидных данных!
                Err(DecodeError::InvalidValue(format!(
                    "Expected 0x00 or 0x01 for Boolean, but got {:#04x}",
                    other_value
                )))
            }
        }
    }
}

impl Encode for Boolean {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            true => writer.push(0x01),
            false => writer.push(0x00),
        }
        Ok(())
    }
}