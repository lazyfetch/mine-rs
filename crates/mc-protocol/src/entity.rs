// temp description of struct
struct Entity {
    entity_id: i32,
    entity_uuid: i32, // uuid data type, todo.
    entity_type: i32,

    x: i16,
    y: i16,
    z: i16,

    pitch: i32, // angle data type, todo.
    yaw: i32, // also
    head_yaw: i32, // also

    data: i32,
    
    velocity_x: i8,
    velocity_y: i8,
    velocity_z: i8,
}

impl Entity {
    fn spawn() {
        
    }

    fn remove() {

    }
}