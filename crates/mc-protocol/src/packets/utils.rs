pub trait Packet {
    type Id;
    const ID: Self::Id;
}
