use crate::packets::types::types::{Encode, EncodeError, UShort};

impl Encode for UShort {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError> {
        // let buf = [0u8; 2];
        let buf: &[u8; 2] = &self.to_be_bytes();
        writer.extend_from_slice(buf);
        Ok(())
    }
}