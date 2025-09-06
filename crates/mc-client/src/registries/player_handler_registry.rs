use crate::{handle_apply_event, packets::clientbound::SynchronizePlayerPositionData, types::MasterHandlers};
use mc_protocol::packets::packet_ids_cb::PlayClientboundPacketId::SynchronizePlayerPosition;
use mc_protocol::player::Player;

use crate::packets::types::{Parse, ProvideTargetKey, ApplyEvent};
use std::io::Cursor;
use std::any::TypeId;


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

impl<'a> PlayerHandlerRegistry<'a> {
    handle_apply_event!(
        on_synchronize,
        SynchronizePlayerPosition,
        Player,
        SynchronizePlayerPositionData,
        Player,
        get_mut_player,
    );

    pub fn on_rotation(&mut self) -> &mut Self {
        self
    }
}