use flate2::{Compression};
use mc_protocol::packets::types::types::{Encode, EncodeError, VarInt};
use std::io::{Write};
use flate2::write::ZlibEncoder;

pub fn encode(payload: &mut Vec<u8>, compression_threshold: i32) -> Result<Vec<u8>, EncodeError> {

    let mut packet: Vec<u8> = Vec::new();

    if compression_threshold < 0 {
        VarInt(payload.len() as i32).encode(&mut packet)?;
        packet.extend_from_slice(payload);
        
    } else if compression_threshold > 0 && payload.len() >= compression_threshold as usize {

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&payload[..])?;
        let compressed_data = encoder.finish()?;
        
        VarInt((VarInt(payload.len() as i32).size() + compressed_data.len()) as i32).encode(&mut packet)?;
        VarInt(payload.len() as i32).encode(&mut packet)?;
        packet.extend_from_slice(&compressed_data);

    } else {
        VarInt((payload.len() + VarInt(0).size()) as i32).encode(&mut packet)?;
        VarInt(0).encode(&mut packet)?;
        packet.extend_from_slice(payload);
    }
    Ok(packet)
}
