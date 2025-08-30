pub trait ApplyEvent<E> {
    fn apply(&mut self, event: E);
}

struct EntityMoveData {
    entity_id: i32, // actually varint
    delta_x: i8,
    delta_y: i8,
    delta_z: i8, //fr i dont know if type is correct, nvrmnd
}

struct EntityRotationData {

}

impl EntityMoveData {
    fn on_move() -> EntityMoveData  {
        EntityMoveData {  }
    }
}

impl ApplyEvent<E> for EntityMoveData {
    fn apply(&mut self, event: E) {
        
    }
}

impl EntityRotationData {
    fn on_rotation() -> EntityRotationData {
        EntityRotationData {  }
    }
}