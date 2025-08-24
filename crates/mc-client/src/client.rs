use super::ClientBuilder;
use super::config;

use std::net::TcpStream;
pub struct Client {
    pub tcp_stream: TcpStream,
}

impl Client {
    pub fn build() -> ClientBuilder {
        ClientBuilder::new()
    }
}