use crate::types::RegistriesMap;

use super::ClientBuilder;
use super::State;

use tokio::net::TcpStream;
// use std::collections::HashMap;

pub struct Client {
    pub username: String,
    pub state: State,
    pub tcp_stream: TcpStream,
    pub registries: RegistriesMap,
}

impl Client {
    pub fn build() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub async fn listen(&mut self) {
        // loop, parse package id -> match -> ...
    }

    pub fn login() {
        // todo
    }

    pub fn login_with_tls() {
        // todo
    }
}