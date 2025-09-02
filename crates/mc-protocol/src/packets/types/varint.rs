use std::io::Read;

use crate::packets::{packet_ids_sb::{ConfigureServerboundPacketId, HandshakeServerboundPacketId, LoginServerboundPacketId, PlayServerboundPacketId}, types::types::{Decode, DecodeError, Encode, EncodeError, Int, VarInt, VARINT_LENGTH}};

impl From<HandshakeServerboundPacketId> for VarInt {
    fn from(packet_id: HandshakeServerboundPacketId) -> Self {
        VarInt(packet_id as i32)
    }
}
impl From<LoginServerboundPacketId> for VarInt {
    fn from(packet_id: LoginServerboundPacketId) -> Self {
        VarInt(packet_id as i32)
    }
}
impl From<ConfigureServerboundPacketId> for VarInt {
    fn from(packet_id: ConfigureServerboundPacketId) -> Self {
        VarInt(packet_id as i32)
    }
}
impl From<PlayServerboundPacketId> for VarInt {
    fn from(packet_id: PlayServerboundPacketId) -> Self {
        VarInt(packet_id as i32)
    }
}

impl Decode for VarInt {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let mut num_read = 0;
        let mut result: Int = 0;
        let mut read_byte = [0u8; 1];
        loop {
            reader.read_exact(&mut read_byte)?;
            let byte = read_byte[0];
            let value = (byte & 0b0111_1111) as Int;
            result |= value << (7 * num_read);
            num_read += 1;
            if num_read > VARINT_LENGTH {
                return Err(DecodeError::InvalidValue("VarInt too long".to_string()));
            }
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok(VarInt(result))
        }
}

impl Encode for VarInt {
    fn encode(&self, writer: &mut Vec<u8>) -> Result<(), EncodeError> {
        let mut value = self.0 as u32;
        loop {
            let mut byte = (value & 0b0111_1111) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0b1000_0000;
            }
            writer.push(byte);
            if value == 0 {
                break;
            }
        }
        Ok(())
    }
}