use std::fmt::Error;

use mc_protocol::packets::{packet_ids_sb::Handshake, types::types::{Double, PrefixedArray, VarInt}};
use tokio::sync::mpsc::Sender;

use crate::{packets::serverbound::HandshakeData, registries::{DataBuilder}};


pub struct PlayerController {
    sender: Sender,
}

impl PlayerController {
    pub fn r#move(x: Double, y: Double, z: Double) -> Result<(), Error> {
        
        // parse packets
        
        // send to channel

        // enjoy the life :)
        // example
        let st = "localhost".to_string();
        let data = HandshakeData {
            protocol_version: VarInt(3),
            server_address: PrefixedArray{
                length: VarInt(st.len() as i32),
                data: st,
            },
            server_port: 25565,
        };
        Handshake::build(data);
        Ok(())
    }

    pub fn move_and_rotate() {

    }
}