use crate::types::PlayHandlers;
use crate::{handle_apply_event, packets::clientbound::SynchronizePlayerPositionData};
use mc_protocol::packets::packet_ids_cb::PlayClientboundPacketId::SynchronizePlayerPosition;
use mc_protocol::player::Player;
use crate::registries::player_storage::PlayerStorage;

use crate::packets::types::{Parse, ProvideTargetKey, ApplyEvent};
use std::io::Cursor;
use std::any::TypeId;

// shitcode for now, temp, check mc-protocol/player.rs why
pub struct PlayerHandlerRegistry<'a> {
    pub play_handlers: &'a mut PlayHandlers 
}

impl<'a> PlayerHandlerRegistry<'a> {
    pub fn new(play_handlers: &'a mut PlayHandlers) -> PlayerHandlerRegistry {
        PlayerHandlerRegistry {
            play_handlers
        }
    }
}

impl<'a> PlayerHandlerRegistry<'a> {
    handle_apply_event!(
        on_synchronize,
        SynchronizePlayerPosition,
        play_handlers,
        PlayerStorage,
        SynchronizePlayerPositionData,
        Player,
        get_mut_player,
    );
}