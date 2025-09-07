use crate::{types::{ConfigureHandlers, LoginHandlers, PlayHandlers}};

pub struct InternalHandlerRegistry<'a> {
    pub login_handlers: &'a mut LoginHandlers,
    pub configure_handlers: &'a mut ConfigureHandlers,
    pub play_handler: &'a mut PlayHandlers,
}

impl <'a>InternalHandlerRegistry<'a>  {
    pub fn new(
        login_handlers: &'a mut LoginHandlers, 
        configure_handler: &'a mut ConfigureHandlers,
        play_handler: &'a mut PlayHandlers) -> InternalHandlerRegistry<'a> {
        
        InternalHandlerRegistry { 
            login_handlers: login_handlers, 
            configure_handlers: configure_handler,
            play_handler: play_handler, 
        }
    }
}