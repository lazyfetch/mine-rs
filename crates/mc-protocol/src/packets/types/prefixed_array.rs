use std::io::Read;

use crate::packets::types::types::{Decode, DecodeError, Encode, EncodeError, PrefixedArray, VarInt};

impl<T: Encode + Decode> Encode for PrefixedArray<T> {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError> {
        VarInt(self.data.len() as i32).encode(writer)?;

        for item in &self.data {
            item.encode(writer)?;
        }

        Ok(())
    }
}

impl<T: Encode + Decode> Decode for PrefixedArray<T> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let length = VarInt::decode(reader)?;
        let len_usize = length.0 as usize;

        let mut data: Vec<T> = Vec::with_capacity(len_usize);

        for _ in 0..len_usize {
            let item = T::decode(reader)?;
            data.push(item);
        }

        Ok(Self {
            length,
            data,
        })
    }
}