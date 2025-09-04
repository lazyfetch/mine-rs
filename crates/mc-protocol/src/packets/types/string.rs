use std::io::Read;

use crate::packets::types::types::{Decode, DecodeError, Encode, EncodeError, StringMC, VarInt, STRING_LENGTH};

impl Encode for StringMC {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError> {
        let string_bytes = self.0.as_bytes();
        let length = string_bytes.len();
        
        if length > STRING_LENGTH {
            return Err(EncodeError::ProtocolViolation("String too long".to_string()));
        }
        VarInt::from(length as i32).encode(writer)?;
        writer.extend_from_slice(string_bytes);
        
        Ok(())
    }
}

impl Decode for StringMC {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let length = VarInt::decode(reader)?;
        let len_usize = length.0 as usize;
        if len_usize > STRING_LENGTH {
            return Err(DecodeError::InvalidValue("String too long".to_string()));
        }
        let mut buf = vec![0u8; len_usize];
        reader.read_exact(&mut buf)?;
        let data = String::from_utf8(buf)?;

        Ok(StringMC(data))
    }
}