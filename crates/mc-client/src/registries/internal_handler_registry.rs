use tokio::sync::mpsc::Sender;
use crate::handle_stateful_event;
use std::any::TypeId;
use crate::packets::clientbound::LoginSuccessData;
use crate::packets::types::{
    DataBuilder, 
    WithReply, 
    ApplyEvent};
use crate::registries::internal_storage::InternalStorage;
use std::io::Cursor;
use crate::packets::types::{Parse};
use crate::{
    handle::Packet, 
    handle_with_reply_event, 
    packets::clientbound::FinishConfigurationData, 
    types::{
        ConfigureHandlers, 
        LoginHandlers, 
        PlayHandlers}};
use mc_protocol::packets::{
    packet_ids_cb::ConfigureClientboundPacketId::FinishConfiguration,
    packet_ids_cb::LoginClientboundPacketId::LoginSuccess, 
    packet_ids_sb::AcknowledgeFinishConfiguration
};
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
    
    pub fn bootstrap(&mut self) {
        self.bootstrap_init();

        // handshake packet

        // login packet

        // LETS FUCKING GOOOOOO!!!!!!!
    }
    
    fn bootstrap_init(&mut self) {
        self.login_success(|| {});
        self.finish_configuration(|_|{});
    }

    handle_with_reply_event!(
        finish_configuration,
        FinishConfiguration,
        configure_handlers,
        FinishConfigurationData,
        AcknowledgeFinishConfiguration,
    );

    handle_stateful_event!(
        login_success,
        LoginSuccess,
        login_handlers,
        InternalStorage,
        LoginSuccessData,
    );
    
}