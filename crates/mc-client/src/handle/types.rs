use crate::handle::player_controller::PlayerController;

pub type Packet = Vec<u8>;

pub trait Controllers {
    fn player_controller(&mut self) -> PlayerController;
}