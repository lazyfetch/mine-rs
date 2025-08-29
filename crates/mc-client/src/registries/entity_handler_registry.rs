use std::{any::TypeId, collections::HashMap};

use crate::{client_builder::MasterHandlers, EntityStorage};
pub struct EntityHandlerRegistry<'a> {
    pub master_handlers: &'a mut MasterHandlers,
}

impl<'a> EntityHandlerRegistry<'a> {

    pub fn new(master_handlers: &'a mut MasterHandlers) -> Self {
        EntityHandlerRegistry {
            master_handlers,
        }
    }

    pub fn on_move<F>(&mut self, user_callback: F) -> &mut Self
    where
        F: FnMut(&mut Entity, MoveData) + 'static, {
        self.master_handlers.insert(MOVE_PACKET_ID, Box::new(move |registries, raw_bytes| {

            let move_data = parse_move_packet(raw_bytes);

            if let Some(storage) = registries.get_mut(&TypeId::of::<EntityStorage>())
                .and_then(|any| any.downcast_mut::<EntityStorage>()) {
                if let Some(entity) = storage.get_entity_mut(move_data.entity_id) {
                    entity.position = move_data.new_position;

                    user_callback(entity, move_data);
                }
            }
        }));

        self
    }

    pub fn on_remove(&mut self) -> &mut Self {
        self
    }
}
