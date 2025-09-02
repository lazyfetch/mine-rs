use std::fmt::Error;

use mc_protocol::types::types::Double;
use tokio::sync::mpsc::Sender;

pub struct PlayerController {
    sender: Sender,
}

impl PlayerController {
    pub fn r#move(x: Double, y: Double, z: Double) -> Result<(), Error> {
        Ok(())
    }

    pub fn move_and_rotate() {

    }
}