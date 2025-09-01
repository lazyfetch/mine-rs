use std::collections::HashMap;
use std::any::{Any, TypeId};
use mc_protocol::packets::packet_ids_cb::PlayClientboundPacketId;

use crate::{EntityHandlerRegistry, PlayerHandlerRegistry};

pub type RegistriesMap = HashMap<TypeId, Box<dyn Any + Send + Sync>>;
pub type MasterHandlers = HashMap<PlayClientboundPacketId, Box<dyn FnMut(&mut RegistriesMap, &[u8]) + 'static>>; // temp, rewrite this dude

pub trait Registry {
    fn entities(&mut self) -> EntityHandlerRegistry;
    fn player(&mut self) -> PlayerHandlerRegistry; 
}