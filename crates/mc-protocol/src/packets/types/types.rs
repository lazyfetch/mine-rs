// This mean varint or varlong cant be more than 32-bit or 64-bit val
pub const VARINT_LENGTH: i8 = 5;
pub const VARLONG_LENGTH: i8 = 10;

// Aliases
pub type Boolean = bool;
pub type Byte = i8;
pub type UByte = u8;
pub type Short = i16;
pub type UShort = u16;
pub type Int = i32;
pub type Long = i64;
pub type Float = f32;
pub type Double = f64;
pub type Angle = u8;
pub type UUID = u128;