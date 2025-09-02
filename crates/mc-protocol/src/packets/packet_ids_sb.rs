use num_enum::TryFromPrimitive;

use crate::packets::{types::types::VarInt, utils::Packet};
 
#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum HandshakeServerboundPacketId {
    Handshake = 0x00,
}

#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum LoginServerboundPacketId {
    Login = 0x00,
}

#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum ConfigureServerboundPacketId {
    ClientInformation = 0x00,
}

#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum PlayServerboundPacketId {
    Some = 0x00,
}

pub struct Handshake;
pub struct Login;

impl Packet for Handshake {
    type Id = HandshakeServerboundPacketId;
    const ID: Self::Id = HandshakeServerboundPacketId::Handshake;
}