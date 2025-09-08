use tokio::{io::AsyncWriteExt, sync::mpsc::{self, Sender}};

use crate::handle::{player_controller::PlayerController, types::Controllers, Packet};

pub struct Handle {
    receiver: mpsc::Receiver<Packet>,
    stream: tokio::net::tcp::OwnedWriteHalf,
}

impl Handle {
    pub fn new(stream: tokio::net::tcp::OwnedWriteHalf) -> Sender<Packet> {
        let (sender, receiver) = mpsc::channel(128);

        let s = Handle {
            receiver: receiver,
            stream: stream,
        };
        
        tokio::spawn(async {
            s.run().await;
        });

        sender
    }

    pub async fn run(mut self) {
        println!("[Handle]: channel run");

        while let Some(packet) = self.receiver.recv().await {
            println!("[Handle]: packet {:?}, send to TCP...", packet);
            self.stream.write_all(&packet).await.unwrap(); // temp
        }
        
        println!("[Handle]: channel closed");
    }

    // for packet, not payload or data, only packet 
    pub fn send(sender: mpsc::Sender<Packet>, packet: Packet) {
        tokio::spawn(async move {
            if let Err(e) = sender.send(packet).await {
                eprintln!("Failed, {}", e);
            }
        });
    }
}

// rewrite
/*
impl Controllers for Handle {
    fn player_controller(&mut self) -> PlayerController {
        PlayerController {
            sender: self.sender.clone(),
        }
    }
}
*/