use tokio::net::TcpStream; 
use tokio::io;

use std::collections::HashMap;
use std::any::{Any, TypeId};

use crate::registries::registries::Registries;
use crate::registries::entity_handler_registry::EntityHandlerRegistry;
use crate::{EntityStorage, State};

use super::config::*;
use super::Client;

pub type RegistriesMap = HashMap<TypeId, Box<dyn Any + Send + Sync>>;
pub type MasterHandlers = HashMap<i32, Box<dyn FnMut(&mut Registries, &[u8]) + 'static>>;

pub trait Registry {
    fn entities(&mut self) -> EntityHandlerRegistry;
    // fn world(&mut self) -> &mut WorldHandlerRegistry;
}

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
            state:       State::Login,
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
        
        Ok(Client {
            username: self.username,
            state: self.state,
            tcp_stream: stream,            
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
}