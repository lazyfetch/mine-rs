// temp description of struct
pub struct Entity {
    pub entity_id: i32,
    pub entity_uuid: i32, // uuid data type, todo.
    pub entity_type: i32,

    pub x: i16,
    pub y: i16,
    pub z: i16,

    pub pitch: i32, // angle data type, todo.
    pub yaw: i32, // also
    pub head_yaw: i32, // also

    pub data: i32,
    
    pub velocity_x: i8,
    pub velocity_y: i8,
    pub velocity_z: i8,
}

impl Entity {
    fn spawn() {
        
    }

    fn remove() {

    }
}