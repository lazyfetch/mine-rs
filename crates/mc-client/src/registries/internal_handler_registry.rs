use tokio::sync::mpsc::Sender;
use crate::handle_stateful_event;
use std::any::TypeId;
use mc_protocol::packets::packet_ids_sb::{KeepAlivePlay as Ka, KeepAliveConfigure as Kc};

use crate::packets::clientbound::{KeepAliveConfigureData, KeepAlivePlayData, LoginSuccessData};
use crate::packets::serverbound::KeepAlivePlayData as KapD;
use crate::packets::types::{
    DataBuilder, 
    WithReply, 
    ApplyEvent};
use crate::packets::encode;
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
    packet_ids_cb::ConfigureClientboundPacketId::{FinishConfiguration},
    packet_ids_cb::LoginClientboundPacketId::LoginSuccess, 
    packet_ids_cb::PlayClientboundPacketId::KeepAlive as Kap,
    packet_ids_cb::ConfigureClientboundPacketId::KeepAlive as Kac,
};
pub struct InternalHandlerRegistry<'a> {
    pub login_handlers: &'a mut LoginHandlers,
    pub configure_handlers: &'a mut ConfigureHandlers,
    pub play_handlers: &'a mut PlayHandlers,
    pub sender: Sender<Packet>,
}

impl <'a>InternalHandlerRegistry<'a>  {
    pub fn new(
        login_handlers: &'a mut LoginHandlers, 
        configure_handlers: &'a mut ConfigureHandlers,
        play_handlers: &'a mut PlayHandlers,
        sender: Sender<Packet>) -> InternalHandlerRegistry<'a> {
        
        InternalHandlerRegistry { 
            login_handlers: login_handlers, 
            configure_handlers: configure_handlers,
            play_handlers: play_handlers,
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
        self.keep_alive_configure(|_|{});
        self.keep_alive_play(|_|{});
        self.login_success(|| {});
        self.finish_configuration(||{});
    }

    handle_stateful_event!(
        finish_configuration,
        FinishConfiguration,
        configure_handlers,
        InternalStorage,
        FinishConfigurationData,
    );

    handle_stateful_event!(
        login_success,
        LoginSuccess,
        login_handlers,
        InternalStorage,
        LoginSuccessData,
    );

    handle_with_reply_event!(
        keep_alive_play,
        Kap,
        play_handlers,
        KeepAlivePlayData,
        Ka,
    );

    handle_with_reply_event!(
        keep_alive_configure,
        Kac,
        configure_handlers,
        KeepAliveConfigureData,
        Kc,
    );
    
}