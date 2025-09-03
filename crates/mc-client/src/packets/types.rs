use std::io::Read;

use mc_protocol::packets::types::types::{DecodeError, EncodeError};

// -- for clientbound.rs --
pub trait ApplyEvent<E> {
    fn apply(&mut self, event: &mut E);
}

pub trait Parse: Sized {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, DecodeError>;
}

pub trait ProvideTargetKey {
    type Key;

    fn key(&self) -> Self::Key;
}
// -- end --

// -- for serverbound.rs -- 
pub trait DataBuilder {
    type Data;

    fn build(data: Self::Data) -> Result<Vec<u8>, EncodeError>;
}
// -- end --