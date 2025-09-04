use mc_protocol::entity::Entity;
use mc_protocol::packets::packet_ids_cb::PlayClientboundPacketId::SpawnEntity;

use crate::packets::types::{Parse, ProvideTargetKey, ApplyEvent};
use crate::packets::clientbound::{EntityMoveData, SpawnEntityData};
use crate::types::MasterHandlers;
use crate::{handle_apply_event, handle_spawn_event, EntityStorage};
use crate::packets::types::SpawnEvent;

use mc_protocol::packets::packet_ids_cb::PlayClientboundPacketId::UpdateEntityPosition;
use std::any::TypeId;

use std::io::Cursor;

// apply? remove? spawn, with_reply (like keep-alive, batch chunks)?...
// its literally HTTP with GET POST DELETE UPDATE LOOL, nvrmd, maybe i need create another macro rules
// and put it into ./types.rs, think about it man

pub struct EntityHandlerRegistry<'a> {
    pub master_handlers: &'a mut MasterHandlers,
}

impl<'a> EntityHandlerRegistry<'a> {

    pub fn new(master_handlers: &'a mut MasterHandlers) -> Self {
        EntityHandlerRegistry {
            master_handlers,
        }
    }

    handle_apply_event!(
        on_move,
        UpdateEntityPosition,
        EntityStorage,
        EntityMoveData,
        Entity,
        get_entity_mut,
    );

    handle_spawn_event!(
        on_spawn,
        SpawnEntity,
        EntityStorage,
        SpawnEntityData,
        Entity,
        get_entity_mut,
    );

    pub fn on_remove(&mut self) -> &mut Self {
        self
    }
}
