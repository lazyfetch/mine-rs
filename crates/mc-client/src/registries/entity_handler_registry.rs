use crate::client_builder::MasterHandlers;

pub struct EntityHandlerRegistry<'a> {
    // here is hashtable
    // format some_uuid | *entity
    pub master_handlers: &'a mut MasterHandlers,
}

impl<'a> EntityHandlerRegistry<'a> {
    pub fn on_spawn() {

    }
    
    pub fn on_move() {

    }

    pub fn on_remove() {

    }
}