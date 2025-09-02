use std::fmt::Error;

use mc_protocol::packets::{packet_ids_sb::Handshake, types::types::{Double, VarInt}};
use tokio::sync::mpsc::Sender;

use crate::{packets::serverbound::HandshakeData, registries::PacketBuilder};


pub struct PlayerController {
    sender: Sender,
}

impl PlayerController {
    pub fn r#move(x: Double, y: Double, z: Double) -> Result<(), Error> {
        
        // parse packets
        
        // send to channel

        // enjoy the life :)
        // example
        let data = HandshakeData {
            protocol_version: VarInt(3),
            server_address: "localhost".to_string(),
            server_port: 25565,
        };
        Handshake::build(data);
        Ok(())
    }

    pub fn move_and_rotate() {

    }
}