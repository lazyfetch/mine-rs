use std::io::Read;
use mc_protocol::{entity::Entity, packets::{packet_ids_sb::{AcknowledgeFinishConfiguration, LoginAcknowledged}, types::types::{Angle, Boolean, Decode, DecodeError, Double, Float, Int, Long, PrefixedArray, Short, StringMC, VarInt, UUID}}, player::Player};

use crate::{handle::handle, packets::{encode, serverbound::{self, AcknowledgeFinishConfigurationData, LoginAcknowledgedData}, types::{ApplyEvent, Parse, ProvideTargetKey}}, registries::{internal_storage::InternalStorage, DataBuilder, RemoveEvent, SpawnEvent, WithReply}, EntityStorage};
use crate::State::Configure;

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

// -- RemoveEntitiesData --
pub struct RemoveEntitiesData {
    pub ids: PrefixedArray<VarInt>
}

impl Parse for RemoveEntitiesData {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        Ok(RemoveEntitiesData { 
            ids: PrefixedArray::<VarInt>::decode(reader)?
        })
    }
}

impl RemoveEvent<EntityStorage> for RemoveEntitiesData {
    fn remove(&mut self, event: &mut EntityStorage) {
        for key in self.ids.data.iter() {
            event.remove(&key.0);
        }
    }
}
// -- RemoveEntitiesData end --

// -- KeepAlivePlayData --
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

// -- KeepAliveplayData end --

// -- SynchronizePlayerPositionData --
// Im not parse flag so i dont know real position and other fields for apply
// its just example
pub struct SynchronizePlayerPositionData {
    teleport_id: VarInt,
    x: Double,
    y: Double,
    z: Double,
    velocity_x: Double,
    velocity_y: Double,
    velocity_z: Double,
    yaw: Float, // not impl
    pitch: Float, // not impl
    flags: Int, // unused i think
}

impl Parse for SynchronizePlayerPositionData {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        Ok(SynchronizePlayerPositionData {
            teleport_id: VarInt::decode(reader)?,
            x: Double::decode(reader)?,
            y: Double::decode(reader)?,
            z: Double::decode(reader)?,
            velocity_x: Double::decode(reader)?,
            velocity_y: Double::decode(reader)?,
            velocity_z: Double::decode(reader)?,
            yaw: Float::decode(reader)?,
            pitch: Float::decode(reader)?,
            flags: Int::decode(reader)?,
        })
    }
}
// example
impl ApplyEvent<Player> for SynchronizePlayerPositionData {
    fn apply(&mut self, event: &mut Player) {
        event.x = self.x;
        event.y = self.y;
        event.z = self.z;
    }
} 

impl ProvideTargetKey for SynchronizePlayerPositionData {
    type Key = i32; // Temp also

    fn key(&self) -> Self::Key {
        0 // Absolutely nvrmd which key we will provide, its temp shit.
    }
}
// -- SynchronizePlayerPositionData end --

// -- Login stage --

pub struct LoginSuccessData {
    uuid: UUID,
    username: StringMC,
    property: LoginSuccessPropertyData,
}

pub struct LoginSuccessPropertyData {
    name: StringMC,
    value: StringMC,
    signature: Option<StringMC>,
}

impl Parse for LoginSuccessData {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let uuid = UUID::decode(reader)?;
        let username = StringMC::decode(reader)?;
        let property = LoginSuccessPropertyData::parse(reader)?;
        Ok(LoginSuccessData {
            uuid: uuid,
            username: username,
            property: property,
        })
    }
}

impl Parse for LoginSuccessPropertyData {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        VarInt::decode(reader)?; // drop useless VarInt - length of prefixed array
        // temp maybe?
        
        let name = StringMC::decode(reader)?;
        let value = StringMC::decode(reader)?;

        let opt = Boolean::decode(reader)?;
        let signature = if opt {
            Some(StringMC::decode(reader)?)
        } else {
            None
        };

        Ok(LoginSuccessPropertyData { 
            name: name, 
            value: value, 
            signature: signature 
        })
    }
}

impl ApplyEvent<InternalStorage> for LoginSuccessData {
    fn apply(&mut self, event: &mut InternalStorage) {
        // Change state, 100% thread safe, cuz its synchonous
        event.state = Configure;
        let sender = event.sender.clone();
        // send to channel package
        let packet_data = LoginAcknowledgedData;
        let mut payload = LoginAcknowledged::build(packet_data).unwrap(); // temp
        let packet = encode::encode(&mut payload, 256).unwrap(); // absolutely shit
        handle::Handle::send(sender, packet);
    }
}

// todo temp
pub struct SetCompressionData {
    pub threshold: VarInt,
}

// -- Login stage end --

// -- Configure stage --
pub struct FinishConfigurationData;

impl Parse for FinishConfigurationData {
    fn parse<R: Read>(_reader: &mut R) -> Result<Self, DecodeError> {
        Ok(FinishConfigurationData)
    }
}

impl ApplyEvent<InternalStorage> for FinishConfigurationData {
    fn apply(&mut self, event: &mut InternalStorage) {
        // 100% i hope
        event.state = crate::State::Play;
        let sender = event.sender.clone();
        let packet_data = AcknowledgeFinishConfigurationData;
        let mut payload = AcknowledgeFinishConfiguration::build(packet_data).unwrap(); // temp
        let packet = encode::encode(&mut payload, 256).unwrap();
        handle::Handle::send(sender, packet);
    }
}

pub struct KeepAliveConfigureData {
    pub id: Long,
}

impl Parse for KeepAliveConfigureData {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        Ok(KeepAliveConfigureData { 
            id: Long::decode(reader)?,
        })
    }
}

impl WithReply for KeepAliveConfigureData {
    type Reply = serverbound::KeepAliveConfigureData;

    fn with_reply(&self) -> Self::Reply {
        serverbound::KeepAliveConfigureData {
            id: self.id
        }
    }
}


// -- Configure stage end --