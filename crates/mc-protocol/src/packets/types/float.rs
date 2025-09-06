use crate::packets::types::types::{Decode, Float};

impl Decode for Float {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, super::types::DecodeError> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        Ok(Float::from_be_bytes(buffer))
    }
}