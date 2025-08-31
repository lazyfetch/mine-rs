use std::io::Read;

use crate::types::types::VARINT_LENGTH;

trait ToVarInt {
    fn to_varint(&self) -> Vec<u8>;
}

impl ToVarInt for i32 {

    fn to_varint(&self) -> Vec<u8> {

        let mut bytes = Vec::new();
        let mut value = *self as u32;
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

fn read_varint<R: Read>(stream: &mut R) -> Result<i32, std::io::Error> {
    
    let mut num_read = 0;
    let mut result = 0;
    let mut read_byte = [0u8; 1];
    loop {
        stream.read_exact(&mut read_byte)?;
        let byte = read_byte[0];
        let value = (byte & 0b0111_1111) as i32;
        result |= value << (7 * num_read);
        num_read += 1;
        if num_read > VARINT_LENGTH {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "VarInt is too big",
            ));
        }
        if (byte & 0b1000_0000) == 0 {
            break;
        }
    }
    Ok(result)
}