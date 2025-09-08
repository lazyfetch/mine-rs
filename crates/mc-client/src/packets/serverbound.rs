use mc_protocol::packets::{packet_ids_sb::{AcknowledgeFinishConfiguration, Handshake, KeepAliveConfigure, KeepAlivePlay, Login, LoginAcknowledged}, types::types::{Boolean, Encode, EncodeError, Long, StringMC, UShort, VarInt}, Packet};

use crate::{registries::{DataBuilder}};

// -- HandshakeData --
pub struct HandshakeData {
    pub protocol_version: VarInt,
    pub server_address: StringMC,
    pub server_port: UShort,
    pub stage: VarInt,
}

impl DataBuilder for Handshake {
    type Data = HandshakeData;

    fn build(data: Self::Data) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt::from(Self::ID).encode(&mut buf)?;
        data.protocol_version.encode(&mut buf)?;
        data.server_address.encode(&mut buf)?;
        data.server_port.encode(&mut buf)?;
        data.stage.encode(&mut buf)?;
        Ok(buf)
    }
}
// -- HandshakeData end --

// -- LoginStartData --
pub struct LoginStartData {
    pub name: StringMC,
    pub uuid_bool: Boolean,
}   

impl DataBuilder for Login {
    type Data = LoginStartData;

    fn build(data: Self::Data) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt::from(Self::ID).encode(&mut buf)?;
        data.name.encode(&mut buf)?;
        data.uuid_bool.encode(&mut buf)?;
        Ok(buf)
    }
}
// -- LoginStartData end --

// -- LoginAcknowledgedData --
pub struct LoginAcknowledgedData;

impl DataBuilder for LoginAcknowledged {
    type Data = LoginAcknowledgedData;

    fn build(_data: Self::Data) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt::from(Self::ID).encode(&mut buf)?;
        Ok(buf)
    }

}
// -- LoginAcknowledgedData end --

// -- KeepAliveData --
pub struct KeepAlivePlayData {
    pub id: Long,
}

impl DataBuilder for KeepAlivePlay {
    type Data = KeepAlivePlayData;
    
    fn build(data: Self::Data) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt::from(Self::ID).encode(&mut buf)?;
        data.id.encode(&mut buf)?;
        Ok(buf)
    }
}
// -- KeepAliveData end --

// -- Configuration stage --

pub struct KeepAliveConfigureData {
    pub id: Long,
}

impl DataBuilder for KeepAliveConfigure {
    type Data = KeepAliveConfigureData;

    fn build(data: Self::Data) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt::from(Self::ID).encode(&mut buf)?;
        data.id.encode(&mut buf)?;
        Ok(buf)
    }
}

pub struct AcknowledgeFinishConfigurationData;

impl DataBuilder for AcknowledgeFinishConfiguration {
    type Data = AcknowledgeFinishConfigurationData;

    fn build(_data: Self::Data) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        VarInt::from(Self::ID).encode(&mut buf)?;  
        Ok(buf)
    }
}

// -- Configuration stage end --