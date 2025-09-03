use crate::packets::types::types::{Decode, Encode, Short};

impl Decode for Short {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, super::types::DecodeError> {
        let mut buffer = [0u8; 2];
        reader.read_exact(&mut buffer)?;
        Ok(Short::from_be_bytes(buffer))
    }
}

impl Encode for Short {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), super::types::EncodeError> {
        let buffer: &[u8; 2] = &self.to_be_bytes();
        writer.extend_from_slice(buffer);
        Ok(())
    }
}