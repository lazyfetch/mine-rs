use std::fmt::Error;

use mc_protocol::packets::{types::types::{Double}};
use tokio::sync::mpsc::{self};

use crate::{handle::Packet};


pub struct PlayerController {
    pub sender: mpsc::Sender<Packet>
}

impl PlayerController {
    pub fn r#move(x: Double, y: Double, z: Double) -> Result<(), Error> {
        Ok(())
    }

    pub fn move_and_rotate() {

    }
}