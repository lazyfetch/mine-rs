use tokio::sync::mpsc::Sender;
use crate::packets::types::DataBuilder;
use crate::packets::types::WithReply;
use std::io::Cursor;
use crate::packets::types::{Parse};
use crate::{handle::Packet, handle_with_reply_event, packets::clientbound::FinishConfigurationData, registries::internal_storage::InternalStorage, types::{ConfigureHandlers, LoginHandlers, PlayHandlers}};
use mc_protocol::packets::{packet_ids_cb::ConfigureClientboundPacketId::FinishConfiguration, packet_ids_sb::AcknowledgeFinishConfiguration};
pub struct InternalHandlerRegistry<'a> {
    pub login_handlers: &'a mut LoginHandlers,
    pub configure_handlers: &'a mut ConfigureHandlers,
    pub play_handler: &'a mut PlayHandlers,
    pub sender: Sender<Packet>,
}

impl <'a>InternalHandlerRegistry<'a>  {
    pub fn new(
        login_handlers: &'a mut LoginHandlers, 
        configure_handler: &'a mut ConfigureHandlers,
        play_handler: &'a mut PlayHandlers,
        sender: Sender<Packet>) -> InternalHandlerRegistry<'a> {
        
        InternalHandlerRegistry { 
            login_handlers: login_handlers, 
            configure_handlers: configure_handler,
            play_handler: play_handler,
            sender: sender, 
        }
    }
    
    fn bootstrap_init(&mut self) {
        // add all packet for handshake-
    }
    handle_with_reply_event!(
        configure_finish,
        FinishConfiguration,
        configure_handlers,
        InternalStorage,
        FinishConfigurationData,
        AcknowledgeFinishConfiguration,
    );
    
    pub fn bootstrap(&mut self) {}
}