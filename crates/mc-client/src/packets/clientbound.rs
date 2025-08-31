use mc_protocol::{entity::Entity, types::types::{Int, Short}};

use crate::packets::types::{ApplyEvent, Parse, ProvideTargetKey};

pub struct EntityMoveData {
    id: Int, // actually varint
    delta_x: Short,
    delta_y: Short,
    delta_z: Short, //fr i dont know if type is correct, nvrmnd
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
        self.id // Возвращаем его
    }
}

impl Parse for EntityRotationData {
    fn parse(reader: &mut impl std::io::Read) -> Result<Self, std::io::Error> {

    }
}