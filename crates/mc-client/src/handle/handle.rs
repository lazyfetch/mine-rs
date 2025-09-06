use tokio::{io::AsyncWriteExt, sync::mpsc};

use crate::handle::{player_controller::PlayerController, types::Controllers, Packet};

pub struct Handle {
    pub sender: mpsc::Sender<Packet>,
    receiver: mpsc::Receiver<Packet>,
    stream: tokio::net::tcp::OwnedWriteHalf,
}

impl Handle {
    pub fn new(stream: tokio::net::tcp::OwnedWriteHalf) -> Handle {
        let (sender, receiver) = mpsc::channel(128);

        Handle {
            sender: sender,
            receiver: receiver,
            stream: stream,
        }  
    }

    pub async fn run(mut self) {
        println!("[Handle]: channel run");

        while let Some(packet) = self.receiver.recv().await {
            println!("[Handle]: packet {:?}, send to TCP...", packet);
            self.stream.write_all(&packet).await;
        }
        
        println!("[Handle]: channel closed");
    }
}

impl Controllers for Handle {
    fn player_controller(&mut self) -> PlayerController {
        PlayerController {
            sender: self.sender.clone(),
        }
    }
}