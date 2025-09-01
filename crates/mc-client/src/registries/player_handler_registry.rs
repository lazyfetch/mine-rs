use crate::types::MasterHandlers;

pub struct PlayerHandlerRegistry<'a> {
    pub master_handlers: &'a mut MasterHandlers
}

impl<'a> PlayerHandlerRegistry<'a> {
    pub fn new(master_handlers: &'a mut MasterHandlers) -> PlayerHandlerRegistry {
        PlayerHandlerRegistry {
            master_handlers
        }
    }
}