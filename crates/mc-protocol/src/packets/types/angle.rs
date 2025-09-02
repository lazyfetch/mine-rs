use crate::packets::types::types::{Angle, Decode};

impl Decode for Angle {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, super::types::DecodeError> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        Ok(Angle::from_be_bytes(buffer))
    }
}