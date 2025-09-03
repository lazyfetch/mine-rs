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
    }

    pub fn move_and_rotate() {

    }
}