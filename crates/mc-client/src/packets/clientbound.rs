use std::io::Read;
use mc_protocol::{entity::Entity, types::types::{Angle, Boolean, Decode, DecodeError, Short, VarInt}};

use crate::packets::types::{ApplyEvent, Parse, ProvideTargetKey};

pub struct EntityMoveData {
    id: VarInt, // actually varint
    delta_x: Short,
    delta_y: Short,
    delta_z: Short,
}

impl Parse for EntityMoveData {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        Ok(EntityMoveData {
            id: VarInt::decode(reader)?,
            delta_x: Short::decode(reader)?,
            delta_y: Short::decode(reader)?,
            delta_z: Short::decode(reader)?,
        })
    }
}

impl ApplyEvent<Entity> for EntityMoveData {
    fn apply(&mut self, event: &mut Entity) {
        event.x += self.delta_x as f64; // cringe moment, as(s) f64
        event.y += self.delta_y as f64; // temp
        event.z += self.delta_z as f64; // temps
    }
}

impl ProvideTargetKey for EntityMoveData {
    type Key = i32; // Temp also

    fn key(&self) -> Self::Key {
        self.id.0 // Temp, be sure it's work correctly, and its logical correctly
    }
}

pub struct EntityRotationData {
    id: VarInt, // actually varint
    yaw: Angle,
    pitch: Angle,
    on_ground: Boolean,
}

impl ApplyEvent<Entity> for EntityRotationData {
    fn apply(&mut self, event: &mut Entity) {
        event.yaw = self.yaw;
        event.pitch = self.pitch;
        event.on_ground = self.on_ground;
    }
}

impl Parse for EntityRotationData {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        Ok(EntityRotationData {
            id: VarInt::decode(reader)?,
            yaw: Angle::decode(reader)?,
            pitch: Angle::decode(reader)?,
            on_ground: Boolean::decode(reader)?,
        })
    }
}