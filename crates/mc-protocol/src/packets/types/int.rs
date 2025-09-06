use crate::packets::types::types::{Decode, Int};

impl Decode for Int {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, super::types::DecodeError> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        Ok(Int::from_be_bytes(buffer))
    }
}