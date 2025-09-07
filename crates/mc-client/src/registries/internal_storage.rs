use tokio::sync::mpsc::Sender;

use crate::{handle::Packet, types::{ConfigureHandlers, LoginHandlers, PlayHandlers}, State::{self, Configure, Login, Play}};

pub enum CurrentHandlers<'a> {
    Login(&'a mut LoginHandlers),
    Configure(&'a mut ConfigureHandlers),
    Play(&'a mut PlayHandlers),
}

pub struct InternalStorage {
    pub login_handlers: LoginHandlers,
    pub configurate_handlers: ConfigureHandlers,
    pub play_handlers: PlayHandlers,
    pub sender: Sender<Packet>,
    pub state: State,
}

impl InternalStorage  {

    pub fn new(sender: Sender<Packet>) -> Self {
        InternalStorage { 
            login_handlers: LoginHandlers::new(), 
            configurate_handlers: ConfigureHandlers::new(), 
            play_handlers: PlayHandlers::new(), 
            state: State::Login,
            sender: sender,
        }
    }
}

impl InternalStorage {
    pub fn current_state_mut(&mut self) -> CurrentHandlers<'_> {
        match self.state {
            Login => CurrentHandlers::Login(&mut self.login_handlers),
            Configure => CurrentHandlers::Configure(&mut self.configurate_handlers),
            Play => CurrentHandlers::Play(&mut self.play_handlers),
        }
    }
}