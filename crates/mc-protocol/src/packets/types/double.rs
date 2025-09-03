use crate::packets::types::types::{Decode, Double};

impl Decode for Double {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, super::types::DecodeError> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        Ok(Double::from_be_bytes(buffer))
    }
}