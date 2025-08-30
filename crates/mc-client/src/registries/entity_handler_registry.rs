use crate::{client_builder::MasterHandlers, EntityStorage};

// todo, dont forget to move this macro rules in more good place
macro_rules! create_event_handler {
    (
        $fn_name:ident,
        $packet_id:expr,
        $registry_type:ty,
        $packet_data_type:ty,
        $target_type:ty,
        $get_target_fn:ident,
    ) => {
        pub fn $fn_name<F>(&mut self, user_callback: F) -> &mut Self 
        where
            $packet_data_type: Parse + ProvideTargetKey,
            F FnMut(&mut $target_type) + 'static {
                self.master_handlers.insert($packet_id, Box::new(move |registries, raw_bytes| {
                    
                    // parse data
                    let mut reader = std::io::Cursor::new(raw_bytes);
                    let packet_data = %packet_data_type::parse(&mut reader).unwrap(); // temp shit

                    // find registry
                    if let Some(registry) = registries.get_mut(&TypeId::of::<$registry_type>())
                        .and_then(|any| any.downcast_mut::<$registry_type>()) {
                            if let Some(target) = registry.$get_target_fn(packet_data.key()) {
                                
                                // apply new info
                                packet_data.ApplyEvent(&mut target)

                                // user callback
                                user_callback(&mut target)
                            }
                        }

            }));
        }
    };
}

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
                    
                    // here's 

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
