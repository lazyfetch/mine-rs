use crate::packets::types::{Parse, ProvideTargetKey, ApplyEvent};
use crate::packets::clientbound::EntityMoveData;
use crate::types::MasterHandlers;
use crate::EntityStorage;

use mc_protocol::packets::packet_ids_cb::PlayClientboundPacketId::UpdateEntityPosition;
use std::any::TypeId;

macro_rules! handle_apply_event {
    (
        $fn_name:ident,
        $packet_id:expr,
        $registry_type:ty,
        $packet_data_type:ty,
        $target_type:ty,
        $get_target_fn:ident,
    ) => {
        pub fn $fn_name<F>(&mut self, mut user_callback: F) -> &mut Self 
        where
            $packet_data_type: Parse + ProvideTargetKey + ApplyEvent<$target_type>,
            F: FnMut(&mut $target_type) + 'static
        {
            self.master_handlers.insert($packet_id, Box::new(move |registries, raw_bytes| {
                    
                    // parse data
                let mut reader = std::io::Cursor::new(raw_bytes);
                let mut packet_data = <$packet_data_type>::parse(&mut reader).unwrap(); // temp shit

                // find registry
                if let Some(registry) = registries.get_mut(&TypeId::of::<$registry_type>())
                    .and_then(|any| any.downcast_mut::<$registry_type>()) {
                        if let Some(mut target) = registry.$get_target_fn(packet_data.key()) {
                                
                            // apply new info
                            packet_data.apply(&mut target);

                            // user callback
                            user_callback(&mut target)
                        }
                    }
            }));
            self
        }
    };
}

pub struct EntityHandlerRegistry<'a> {
    pub master_handlers: &'a mut MasterHandlers,
}

impl<'a> EntityHandlerRegistry<'a> {

    handle_apply_event!(
        on_move,
        UpdateEntityPosition,
        EntityStorage,
        EntityMoveData,
        mc_protocol::entity::Entity,
        get_entity_mut,
    );

    pub fn new(master_handlers: &'a mut MasterHandlers) -> Self {
        EntityHandlerRegistry {
            master_handlers,
        }
    }

    pub fn on_remove(&mut self) -> &mut Self {
        self
    }
}
