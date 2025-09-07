use std::any::TypeId;
use std::io::Cursor;
use std::io::Read;
use crate::handle::handle::Handle;
use crate::packets::decode::decode_packet;
use crate::registries::internal_storage::CurrentHandlers;
use crate::registries::internal_storage::InternalStorage;
use crate::types::RegistriesMap;


use super::ClientBuilder;
use super::State;
use mc_protocol::packets::packet_ids_cb::ConfigureClientboundPacketId;
use mc_protocol::packets::packet_ids_cb::LoginClientboundPacketId;
use mc_protocol::packets::packet_ids_cb::PlayClientboundPacketId;
use mc_protocol::packets::types::types::Decode;
use mc_protocol::packets::types::types::VarInt;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;

pub struct Client {
    pub username: String,
    pub state: State,
    pub compression: i32,

    pub handle: Handle,
    pub read: OwnedReadHalf,
    pub registries: RegistriesMap,
}

impl Client {
    pub fn build() -> ClientBuilder {
        ClientBuilder::new()
    }
    /*
    pub async fn run(&mut self) -> (&mut Self, Handle){
        let handle = Handle::new(self.write);
        
        
        (self, handle)
    }*/
    
    // this all look like shit...
    async fn read(&mut self) {
        let mut buffer = Vec::with_capacity(4096);

        loop {
            let mut read_buf = [0u8; 4096];
            let bytes_read = self.read.read(&mut read_buf).await.unwrap(); // temp shit
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
                let uncompressed_data = decode_packet(&mut packet_cursor, self.compression).unwrap(); // temp shit
                let mut data_cursor = Cursor::new(&uncompressed_data);
                let packet_id = VarInt::decode(&mut data_cursor).unwrap(); // temp shit?
                let mut raw_data = Vec::new(); // shit temp
                Read::read_to_end(&mut data_cursor,&mut raw_data).unwrap(); // temp shit
                if let Some(storage) = self.registries.get_mut(&TypeId::of::<InternalStorage>()).
                    and_then(|any| any.downcast_mut::<InternalStorage>()) {
                        let state_enum = storage.current_state();
                        match state_enum {
                            CurrentHandlers::Login(login_handlers) => {
                                if let Ok(login_packet_id) = LoginClientboundPacketId::try_from(packet_id.0) {
                                    if let Some(mut handler) = login_handlers.remove(&login_packet_id) {
                                        (handler.as_mut())(&mut self.registries, &raw_data[..]);
                                        login_handlers.insert(login_packet_id, handler);
                                    }
                                }
                            }
                            CurrentHandlers::Configure(config_handlers) => {
                                if let Ok(config_packet_id) = ConfigureClientboundPacketId::try_from(packet_id.0) {
                                    if let Some(mut handler) = config_handlers.remove(&config_packet_id) {
                                        (handler.as_mut())(&mut self.registries, &raw_data[..]);
                                    }
                                }
                            }
                            CurrentHandlers::Play(play_handlers) => {
                                if let Ok(play_packet_id) = PlayClientboundPacketId::try_from(packet_id.0) {
                                    if let Some(mut handler) = play_handlers.remove(&play_packet_id) {
                                        (handler.as_mut())(&mut self.registries, &raw_data[..]);
                                    }
                                }
                            }
                        }
                    }
                Read::read_to_end(&mut data_cursor,&mut raw_data).unwrap(); // temp shit
                

                cursor.set_position(end_of_packet_pos as u64);
            }
            let bytes_processed = cursor.position() as usize;
            buffer.drain(..bytes_processed);
        }
    }
}