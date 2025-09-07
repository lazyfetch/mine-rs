use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::sync::Arc;
use mc_protocol::packets::packet_ids_cb::{ConfigureClientboundPacketId, LoginClientboundPacketId, PlayClientboundPacketId};
use std::sync::Mutex;


use crate::registries::internal_handler_registry::InternalHandlerRegistry;
use crate::{EntityHandlerRegistry, PlayerHandlerRegistry};


pub type RegistriesMap = HashMap<TypeId, Box<dyn Any>>;

pub type Handler = Box<dyn FnMut(&mut RegistriesMap, &[u8]) + 'static>;

pub type PlayHandlers = HashMap<PlayClientboundPacketId, Handler>; // temp, rewrite this dude
pub type LoginHandlers = HashMap<LoginClientboundPacketId, Handler>;
pub type ConfigureHandlers = HashMap<ConfigureClientboundPacketId, Handler>;


pub trait Registry {
    fn entities(&mut self) -> EntityHandlerRegistry;
    fn player(&mut self) -> PlayerHandlerRegistry; 
    // fn internal(&mut self) -> InternalHandlerRegistry;
}

pub trait Internal {
    fn internal(&mut self) -> InternalHandlerRegistry;
}