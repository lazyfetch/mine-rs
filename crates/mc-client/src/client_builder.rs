use mc_protocol::player::Player;
use tokio::net::TcpStream; 
use tokio::io;
use crate::handle::handle::Handle;
use std::collections::HashMap;
use std::any::{TypeId};

use crate::registries::entity_handler_registry::EntityHandlerRegistry;
use crate::types::{MasterHandlers, RegistriesMap, Registry};
use crate::{EntityStorage, PlayerHandlerRegistry, State};

use super::config::*;
use super::Client;

pub struct ClientBuilder {
    host: String,
    port: u16,
    username: String,
    compression: i32,
    state: State,
    master_handlers: MasterHandlers,
    registries: RegistriesMap,
}

impl ClientBuilder {

    pub fn new() -> Self {
        Self {
            host:        DEFAULT_SERVER_HOST.to_string(),
            port:        DEFAULT_SERVER_PORT,
            username:    DEFAULT_USERNAME.to_string(),
            compression: DEFAULT_COMPRESSION_THRESHOLD,
            state:       State::Handshake,
            master_handlers: HashMap::new(),
            registries: HashMap::new(),
        }
    }

    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }
    
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }
    
    pub fn with_compression(mut self, threshold: i32) -> Self {
        self.compression = threshold;
        self
    }

    pub async fn connect(self) -> io::Result<Client> {
        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).await?; // temp  
        let (read, write) = stream.into_split();
        
        let handle = Handle::new(write);

        Ok(Client {
            username: self.username,
            state: self.state,
            read: read,
            handle: handle,
            master_handlers: self.master_handlers,
            registries: self.registries,
            compression: self.compression,
        })
    } 
}

impl Registry for ClientBuilder {
    fn entities(&mut self) -> EntityHandlerRegistry {
        self.registries
            .entry(TypeId::of::<EntityStorage>())
            .or_insert_with(|| Box::new(EntityStorage::default()));

        EntityHandlerRegistry::new(&mut self.master_handlers)
    }

    fn player(&mut self) -> PlayerHandlerRegistry {
        // im not sure its right, temp
        self.registries
            .entry(TypeId::of::<Player>())
            .or_insert_with(|| Box::new(Player::default()));

        PlayerHandlerRegistry::new(&mut self.master_handlers)
    }
}