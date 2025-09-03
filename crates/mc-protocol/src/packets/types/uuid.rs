use crate::packets::types::types::{Decode, UUID};

impl Decode for UUID {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, super::types::DecodeError> {
        let mut buffer = [0u8; 16];
        reader.read_exact(&mut buffer)?;
        Ok(UUID::from_be_bytes(buffer))
    }
}