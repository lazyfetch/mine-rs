use std::io::Read;

fn read_boolean<R: Read>(stream: &mut R) -> Result<bool, std::io::Error> {
    let mut read_byte = [0u8; 1];
    stream.read_exact(&mut read_byte)?;
    if read_byte[0] == 0x01 {
        Ok(true)
    } else if read_byte[0] == 0x00 {
        Ok(false)
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "not boolean",
        ));
    }
}