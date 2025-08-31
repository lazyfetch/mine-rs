use crate::types::types::{Angle, Double, Int, Short, UUID};

// temp description of struct
pub struct Entity {
    pub id: Int,
    pub uuid: UUID, 
    pub r#type: Int,

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
}