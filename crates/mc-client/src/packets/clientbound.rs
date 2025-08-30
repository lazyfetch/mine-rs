use mc_protocol::entity::Entity;

use crate::packets::types::{ApplyEvent, Parse, ProvideTargetKey};

pub struct EntityMoveData {
    entity_id: i32, // actually varint
    delta_x: i16,
    delta_y: i16,
    delta_z: i16, //fr i dont know if type is correct, nvrmnd
}

pub struct EntityRotationData {

}

impl Parse for EntityMoveData {
    fn parse(reader: &mut impl std::io::Read) -> Result<Self, std::io::Error> {
        // lets parse it!
    }
}

impl ApplyEvent<Entity> for EntityMoveData {
    fn apply(&mut self, event: &mut Entity) {
        event.x += self.delta_x;
        event.y += self.delta_y;
        event.z += self.delta_z;
    }
}

impl ProvideTargetKey for EntityMoveData {
    type Key = i32; // Указываем конкретный тип ключа

    fn key(&self) -> Self::Key {
        self.entity_id // Возвращаем его
    }
}

impl Parse for EntityRotationData {
    fn parse(reader: &mut impl std::io::Read) -> Result<Self, std::io::Error> {

    }
}