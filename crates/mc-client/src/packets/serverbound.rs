// 0x00 handshaking
// protocol version - VarInt (772)
// Server address - string (255) 127.0.0.1 like u know
// Port - ushort 
// Intent - VarInt Enum 1. Status 2. Login 3. Transfer (2 for us)
// 0x00 login
// name - String(16)
// uuid - uuid (unused)
// 0x01 skip, cuz we dont use encryption for now
// 0x02 also
// 0x03 login acknowledged
// 0x00 configure. Well must next packets need to ping-pong meanings so server send some shit to client, client response
// its mean login_handler_registry or something else to boostrap our client, make all steps and success coming on Login stage

use mc_protocol::packets::{packet_ids_sb::{Handshake, Login, LoginAcknowledged}, types::types::{Encode, EncodeError, StringMC, UShort, VarInt}, Packet};

use crate::registries::DataBuilder;

pub struct HandshakeData {
    pub protocol_version: VarInt,
    pub server_address: StringMC,
    pub server_port: UShort,
}

impl DataBuilder for Handshake {
    type Data = HandshakeData;

    fn build(data: Self::Data) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt::from(Self::ID).encode(&mut buf)?;
        data.protocol_version.encode(&mut buf)?;
        data.server_address.encode(&mut buf)?;
        data.server_port.encode(&mut buf)?;
        Ok(buf)
    }
}

pub struct LoginStartData {
    pub name: StringMC,
    // player_uuid: UUID // unused 
}

impl DataBuilder for Login {
    type Data = LoginStartData;

    fn build(data: Self::Data) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt::from(Self::ID).encode(&mut buf)?;
        data.name.encode(&mut buf)?;
        Ok(buf)
    }
}

pub struct LoginAcknowledgedData;

impl DataBuilder for LoginAcknowledged {
    type Data = LoginAcknowledgedData;

    fn build(_data: Self::Data) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt::from(Self::ID).encode(&mut buf)?;
        Ok(buf)
    }

}