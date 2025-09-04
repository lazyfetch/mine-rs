use mc_protocol::packets::types::types::{Decode, DecodeError, VarInt};
use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn decode_packet<R: Read>(reader: &mut R, compression_threshold: i32) -> Result<Vec<u8>, DecodeError> {
    
    if compression_threshold < 0 {
        let packet_length = VarInt::decode(reader)?;
        let mut payload = vec![0u8; packet_length.0 as usize];
        reader.read_exact(&mut payload)?;
        
        Ok(payload)

    } else {
        let packet_length = VarInt::decode(reader)?;
        let mut limited_reader = reader.take(packet_length.0 as u64);
        let data_length = VarInt::decode(&mut limited_reader)?;
        let data_len_usize = data_length.0 as usize;

        if data_len_usize == 0 {
            let mut payload = Vec::new();
            limited_reader.read_to_end(&mut payload)?;
            Ok(payload)
        } else {
            let mut decoder = ZlibDecoder::new(limited_reader);
            let mut payload = Vec::with_capacity(data_len_usize);
            decoder.read_to_end(&mut payload)?;
            
            if payload.len() != data_len_usize {
                return Err(DecodeError::InvalidValue(format!(
                    "Decompression failed: expected {} bytes, got {}",
                    data_len_usize,
                    payload.len()
                )));
            }
            Ok(payload)
        }
    }
}