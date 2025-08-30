pub trait ApplyEvent<E> {
    fn apply(&mut self, event: &mut E);
}

pub trait Parse: Sized {
    fn parse(reader: &mut impl std::io::Read) -> Result<Self, std::io::Error>;
}

pub trait ProvideTargetKey {
    type Key;

    fn key(&self) -> Self::Key;
}