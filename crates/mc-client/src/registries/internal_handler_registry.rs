use mc_protocol::packets::types::types::{StringMC, VarInt};
use tokio::sync::mpsc::Sender;
use crate::handle_stateful_event;
use crate::packets::serverbound::{HandshakeData, LoginStartData};
use std::any::TypeId;
use mc_protocol::packets::packet_ids_sb::{Handshake, KeepAliveConfigure as Kc, KeepAlivePlay as Ka, Login};

use crate::packets::clientbound::{KeepAliveConfigureData, KeepAlivePlayData, LoginSuccessData};
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

        // handshake packet temp shit whatever nvrmd
        let hs = HandshakeData {
            protocol_version: VarInt(767),
            server_address: StringMC("localhost".to_string()),
            server_port: 25565,
        };
        let mut payload_hs = Handshake::build(hs).unwrap();
        let packet_hs = encode::encode(&mut payload_hs, -1).unwrap();//temp
        // shit but temp whatever lol
        let clone_hs = self.sender.clone();
        tokio::spawn(async move {
            if let Err(e) = clone_hs.send(packet_hs).await {
                eprintln!("Failed, {}", e);
            }
        });

        // login packet
        let lg = LoginStartData {
            name: StringMC("superded".to_string()),
        };
        let mut payload_lg = Login::build(lg).unwrap(); // temp shiii
        let packet_lg = encode::encode(&mut payload_lg, -1).unwrap(); // temp
        let clone_lg = self.sender.clone();
        tokio::spawn(async move {
            if let Err(e) = clone_lg.send(packet_lg).await {
                eprintln!("Failed, {}", e);
            }
        });
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