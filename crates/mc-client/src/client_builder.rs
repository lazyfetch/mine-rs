use std::net::TcpStream;

use super::config::*;

use super::Client;

pub struct ClientBuilder {
    host: String,
    port: u16,
    username: String,
    compression: i32
}

impl ClientBuilder {

    pub fn new() -> Self {
        Self {
            host:        DEFAULT_SERVER_HOST.to_string(),
            port:        DEFAULT_SERVER_PORT,
            username:    DEFAULT_USERNAME.to_string(),
            compression: DEFAULT_COMPRESSION_THRESHOLD,
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

    pub fn connect(self) -> Client {
        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).unwrap(); // temp  
        
        Client {
            tcp_stream: stream,            
        }

    } 
}