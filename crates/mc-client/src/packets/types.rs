use std::io::Read;

use mc_protocol::types::types::DecodeError;

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