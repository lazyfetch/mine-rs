use mc_protocol::player::Player;

pub struct PlayerStorage {
    player: Player,
}


impl PlayerStorage {
    pub fn get_mut_player(&mut self, _key: i32) -> Option<&mut Player> {
        Some(&mut self.player)
    }
}