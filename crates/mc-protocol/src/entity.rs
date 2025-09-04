use crate::packets::types::types::{Angle, Boolean, Double, Short, VarInt, UUID};

// temp description of struct
pub struct Entity {
    pub id: VarInt,
    pub uuid: UUID, 
    pub r#type: VarInt,

    pub x: Double,
    pub y: Double,
    pub z: Double,

    pub pitch: Angle,
    pub yaw: Angle,
    pub head_yaw: Angle,

    pub data: VarInt,
    
    pub velocity_x: Short,
    pub velocity_y: Short,
    pub velocity_z: Short,

    pub on_ground: Boolean, // dont sure its need here, todo
}