use crate::packets::types::types::{Decode, Encode, EncodeError, Long};

impl Decode for Long {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, super::types::DecodeError> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        Ok(Long::from_be_bytes(buffer))
    }
}

impl Encode for Long {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError> {
        let buf: &[u8; 8] = &self.to_be_bytes();
        writer.extend_from_slice(buf);
        Ok(())
    }
}