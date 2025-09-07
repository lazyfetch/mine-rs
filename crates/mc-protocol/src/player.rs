use crate::packets::types::types::{Angle, Boolean, Double, Int, Short, UUID};

// player is not primivite dude, so temp
#[derive(Default)]
pub struct Player {
    pub id: Int,
    pub uuid: UUID, 

    pub x: Double,
    pub y: Double,
    pub z: Double,

    pub pitch: Angle,
    pub yaw: Angle,
    pub head_yaw: Angle,

    pub data: Int,
    
    pub velocity_x: Short,
    pub velocity_y: Short,
    pub velocity_z: Short,

    pub on_ground: Boolean, // dont sure its need here, todo temp
}

// absolutely shitcode, i shouldn't link mc-protocol and mc-client, but... i just want to run the code.
// It would be good to make player_storage and wrap it (i mean wrap player struct in player_storage struct), but okay, temp.
/*impl Player {
    pub fn get_mut_player(&mut self, _key: i32) -> Option<&mut Player> {
        Some(self)
    }
}*/