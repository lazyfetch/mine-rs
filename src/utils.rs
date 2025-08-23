use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

fn compress_bytes(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

trait ToVarInt {
    fn to_varint(&self) -> Vec<u8>;
}

impl ToVarInt for i32 {
    fn to_varint(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut value = *self as u32; // Работаем с unsigned для сдвигов

        loop {
            let mut byte = (value & 0b0111_1111) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0b1000_0000;
            }
            bytes.push(byte);

            if value == 0 {
                break;
            }
        }
        bytes
    }
}

pub fn packet_builder(compression_threshold: i32, packet_id: i32, payload: Vec<u8>) -> Vec<u8> {
    
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(&packet_id.to_varint()[..]);
    data.extend_from_slice(&payload[..]);
    
    let packet = if compression_threshold >= 0 {
        
        let mut data_length: Vec<u8> = Vec::new();

        if data.len() as i32 >= compression_threshold {
            
            let uncompressed_data_length = data.len() as i32;
            data_length.extend_from_slice(&uncompressed_data_length.to_varint());
            // compress
            let compressed_data = compress_bytes(&data[..]);
            data = compressed_data

        } else {
            data_length.extend_from_slice(&0.to_varint()[..]);
        }
        let packet_length = data.len() as i32;
        let mut packet = Vec::new();

        packet.extend_from_slice(&packet_length.to_varint()[..]);
        packet.extend_from_slice(&data_length[..]);
        packet.extend_from_slice(&data[..]);

        packet
    } else {
        let mut packet: Vec<u8> = Vec::new();
        let packet_length = data.len() as i32;
        packet.extend_from_slice(&packet_length.to_varint()[..]);
        packet.extend_from_slice(&data[..]);
        
        packet
    };

    packet


}