use std::io::{Read};

use crate::packets::types::types::{Decode, DecodeError, Encode, EncodeError, StringMC, VarInt, STRING_LENGTH};

impl Decode for String {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let mut data: Vec<u8> = Vec::new();
        reader.read_to_end(&mut data)?;
        let data = String::from_utf8(data)?;
        Ok(data)
    }
}

impl Encode for String {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError> {
        if self.len() > STRING_LENGTH {
            return Err(EncodeError::ProtocolViolation("String too long".to_string()));
        } else {
            writer.extend_from_slice(self.as_bytes());
            Ok(())
        }
    }
}

impl Encode for StringMC {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.length.encode(writer)?;
        self.data.encode(writer)?;
        Ok(())
    }
}

impl Decode for StringMC {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let length = VarInt::decode(reader)?;
        let mut buf = vec![0u8; length.0 as usize]; 
        reader.read_exact(&mut buf)?;
        let mut cursor: &[u8] = &buf;
        let data = String::decode(&mut cursor)?;
        Ok(StringMC {
            length: length,
            data: data,
        })
    }
}