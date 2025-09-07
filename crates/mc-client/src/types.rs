use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::sync::Arc;
use mc_protocol::packets::packet_ids_cb::{ConfigureClientboundPacketId, LoginClientboundPacketId, PlayClientboundPacketId};
use tokio::sync::RwLock;

use crate::registries::internal_handler_registry::InternalHandlerRegistry;
use crate::{EntityHandlerRegistry, PlayerHandlerRegistry};


pub type RegistriesMap = HashMap<TypeId, Box<dyn Any + Send + Sync>>;

pub type Handler = Box<dyn Fn(Arc<RwLock<RegistriesMap>>, &[u8]) + Send + Sync>;

pub type PlayHandlers = HashMap<PlayClientboundPacketId, Handler>; // temp, rewrite this dude
pub type LoginHandlers = HashMap<LoginClientboundPacketId, Handler>;
pub type ConfigureHandlers = HashMap<ConfigureClientboundPacketId, Handler>;


pub trait Registry {
    fn entities(&mut self) -> EntityHandlerRegistry;
    fn player(&mut self) -> PlayerHandlerRegistry; 
    fn internal(&mut self) -> InternalHandlerRegistry;
}