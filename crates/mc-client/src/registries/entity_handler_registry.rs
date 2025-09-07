use mc_protocol::entity::Entity;
use mc_protocol::packets::packet_ids_cb::PlayClientboundPacketId::{SpawnEntity, RemoveEntities};
use crate::packets::types::RemoveEvent;

use crate::packets::types::{Parse, ProvideTargetKey, ApplyEvent};
use crate::packets::clientbound::{EntityMoveData, RemoveEntitiesData, SpawnEntityData};
use crate::types::PlayHandlers;
use crate::{handle_apply_event, handle_remove_event, handle_spawn_event, EntityStorage};
use crate::packets::types::SpawnEvent;

use mc_protocol::packets::packet_ids_cb::PlayClientboundPacketId::UpdateEntityPosition;
use std::any::TypeId;

use std::io::Cursor;

// apply? remove? spawn, with_reply (like keep-alive, batch chunks)?...
// its literally HTTP with GET POST DELETE UPDATE LOOL, nvrmd, maybe i need create another macro rules
// and put it into ./types.rs, think about it man

pub struct EntityHandlerRegistry<'a> {
    pub play_handlers: &'a mut PlayHandlers,
}

impl<'a> EntityHandlerRegistry<'a> {

    pub fn new(play_handlers: &'a mut PlayHandlers) -> Self {
        EntityHandlerRegistry {
            play_handlers,
        }
    }

    handle_apply_event!(
        on_move,
        UpdateEntityPosition,
        play_handlers,
        EntityStorage,
        EntityMoveData,
        Entity,
        get_entity_mut,
    );

    handle_spawn_event!(
        on_spawn,
        SpawnEntity,
        play_handlers,
        EntityStorage,
        SpawnEntityData,
        Entity,
        get_entity_mut,
    );

    handle_remove_event!(
        on_remove,
        RemoveEntities,
        play_handlers,
        EntityStorage,
        RemoveEntitiesData,
    );
}
