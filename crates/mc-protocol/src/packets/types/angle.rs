use crate::packets::types::types::{Angle, Decode, Encode};

impl Decode for Angle {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, super::types::DecodeError> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        Ok(Angle::from_be_bytes(buffer))
    }
}

impl Encode for Angle {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), super::types::EncodeError> {
        let buffer: &[u8; 1] = &self.to_be_bytes();
        writer.extend_from_slice(buffer);
        Ok(())
    }
}