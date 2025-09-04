use std::io::Read;
use mc_protocol::{entity::{self, Entity}, packets::types::types::{Angle, Boolean, Decode, DecodeError, Double, Long, Short, VarInt, UUID}};

use crate::{packets::{serverbound, types::{ApplyEvent, Parse, ProvideTargetKey}}, registries::{SpawnEvent, WithReply}, EntityStorage};


// -- EntityMoveData --
pub struct EntityMoveData {
    id: VarInt,
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
// -- EntityMoveData end --

// -- EntityRotationData --
pub struct EntityRotationData {
    id: VarInt,
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

impl ProvideTargetKey for EntityRotationData {
    type Key = i32; // temp maybe

    fn key(&self) -> Self::Key {
        self.id.0
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
// -- EntityRotationData end --

// -- SpawnEntityData --
pub struct SpawnEntityData {
    id: VarInt,
    uuid: UUID,
    r#type: VarInt,
    x: Double,
    y: Double,
    z: Double,
    pitch: Angle,
    yaw: Angle,
    head_yaw: Angle,
    data: VarInt,
    velocity_x: Short,
    velocity_y: Short,
    velocity_z: Short,
} 

impl SpawnEntityData {
    fn into_entity(&self) -> Entity {
        Entity {
            id: self.id,
            uuid: self.uuid,
            r#type: self.r#type,
            x: self.x,
            y: self.y,
            z: self.z,
            pitch: self.pitch,
            yaw: self.yaw,
            head_yaw: self.head_yaw,
            data: self.data,
            velocity_x: self.velocity_x,
            velocity_y: self.velocity_y,
            velocity_z: self.velocity_z,
            on_ground: true, // temp shit absolutely temp shit
        }
    }
}

impl ProvideTargetKey for SpawnEntityData {
    type Key = i32;
    
    fn key(&self) -> Self::Key {
        self.id.0 // temp??? temp
    }
}

impl Parse for SpawnEntityData {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        Ok(SpawnEntityData { 
            id: VarInt::decode(reader)?, 
            uuid: UUID::decode(reader)?, 
            r#type: VarInt::decode(reader)?, 
            x: Double::decode(reader)?, 
            y: Double::decode(reader)?, 
            z: Double::decode(reader)?, 
            pitch: Angle::decode(reader)?, 
            yaw: Angle::decode(reader)?, 
            head_yaw: Angle::decode(reader)?, 
            data: VarInt::decode(reader)?, 
            velocity_x: Short::decode(reader)?, 
            velocity_y: Short::decode(reader)?, 
            velocity_z: Short::decode(reader)?, 
        })
    }
}

impl SpawnEvent<EntityStorage> for SpawnEntityData {
    fn spawn(&mut self, event: &mut EntityStorage) {
        let entity = self.into_entity(); 
        event.add(entity);
    }
}
// -- SpawnEntityData end

// -- KeepAliveData --
pub struct KeepAlivePlayData {
    id: Long,
}

impl Parse for KeepAlivePlayData {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        Ok(KeepAlivePlayData { 
            id: Long::decode(reader)? 
        })
    }
}

impl WithReply for KeepAlivePlayData {
    type Reply = serverbound::KeepAlivePlayData;

    fn with_reply(&self) -> Self::Reply {
        serverbound::KeepAlivePlayData {
            id: self.id,
        }
    }
}