use crate::types::types::{Decode, Short};

impl Decode for Short {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, super::types::DecodeError> {
        let mut buffer = [0u8; 2];
        reader.read_exact(&mut buffer)?;
        Ok(Short::from_be_bytes(buffer))
    }
}