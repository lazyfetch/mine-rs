use std::io::Read;

use crate::types::types::{Decode, DecodeError, Encode, EncodeError, Long, VarLong, VARLONG_LENGTH};

impl Decode for VarLong {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let mut num_read = 0;
        let mut result: Long = 0;
        let mut read_byte = [0u8; 1];
        loop {
            reader.read_exact(&mut read_byte)?;
            let byte = read_byte[0];
            let value = (byte & 0b0111_1111) as Long;
            result |= value << (7 * num_read);
            num_read += 1;
            if num_read > VARLONG_LENGTH {
                return Err(DecodeError::InvalidValue("VarLong too long".to_string()));
            }
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok(VarLong(result))
        }
}

impl Encode for VarLong {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError> {
        let mut value = self.0 as u64;
        loop {
            let mut byte = (value & 0b0111_1111) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0b1000_0000;
            }
            writer.push(byte);
            if value == 0 {
                break;
            }
        }
        Ok(())
    }
}