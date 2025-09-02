use std::io::Cursor;
use std::io::Read;
use crate::types::MasterHandlers;
use crate::types::RegistriesMap;

use super::ClientBuilder;
use super::State;
use flate2::bufread::ZlibDecoder;
use mc_protocol::types::types::Decode;
use mc_protocol::types::types::Int;
use mc_protocol::types::types::VarInt;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct Client {
    pub username: String,
    pub state: State,
    pub tcp_stream: TcpStream,
    pub master_handlers: MasterHandlers,
    pub registries: RegistriesMap,
    pub compression: i32,
}

impl Client {
    pub fn build() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub async fn run(&mut self) {
        // loop, parse package id -> match -> ...
        // this function will start the parse of clientbound packets
        // tokio::spawn(self.parse());
        /* some temp buffer, o 
        loop {

        } */
    }

    async fn parse(&mut self) {
        let mut buffer = Vec::with_capacity(4096);

        loop {
            let mut read_buf = [0u8; 4096];
            let bytes_read = self.tcp_stream.read(&mut read_buf).await.unwrap(); // temp shit
            if bytes_read == 0 {
                panic!("connection closed") // look like temp shit
            }
            buffer.extend_from_slice(&read_buf[..bytes_read]);
            let mut cursor = Cursor::new(&buffer);
            'parse: loop {
                let initial_pos = cursor.position();

                let packet_length = match VarInt::decode(&mut cursor) {
                    Ok(len) => len,
                    Err(_) => break 'parse, // not enought len
                };
                let end_of_packet_pos = cursor.position() as i32 + packet_length.0; // temp shit?
                if end_of_packet_pos as u64 > buffer.len() as u64 {
                    cursor.set_position(initial_pos);
                    break 'parse;
                }
                let packet_data_start = cursor.position() as usize;
                let packet_data_end = end_of_packet_pos as usize;
                let packet_data = &cursor.get_ref()[packet_data_start..packet_data_end];
                let mut packet_cursor = Cursor::new(packet_data);
                let uncompressed_data = if self.compression >= 0 {
                    let data_lenght = VarInt::decode(&mut packet_cursor).unwrap(); // temp shit?
                    if data_lenght.0 == 0 { // temp shit?
                        let mut data = Vec::new();
                        Read::read_to_end(&mut packet_cursor,&mut data).unwrap(); // temp shit?
                        data
                    } else {
                        let mut decoder = ZlibDecoder::new(packet_cursor);
                        let mut decompressed = Vec::new();
                        decoder.read_to_end(&mut decompressed).unwrap(); // temp shit
                        decompressed
                    }
                } else {
                    packet_data.to_vec()
                };
                let mut data_cursor = Cursor::new(&uncompressed_data);
                let packet_id = VarInt::decode(&mut data_cursor).unwrap(); // temp shit?

                // match with master_handlers map and find closure, run, and have fun...
            }
            let bytes_processed = cursor.position() as usize;
            buffer.drain(..bytes_processed);
        }
    }
}