#[derive(Debug, Eq, PartialEq, Hash, /*TryFromPrimitive*/)]
#[repr(i32)]
enum LoginServerboundPacketId {
    Login = 0x00,
}

#[derive(Debug, Eq, PartialEq, Hash, /*TryFromPrimitive*/)]
#[repr(i32)]
enum ConfigureServerboundPacketId {
    ClientInformation = 0x00,
}

#[derive(Debug, Eq, PartialEq, Hash, /*TryFromPrimitive*/)]
#[repr(i32)]
enum PlayServerboundPacketId {
    Some = 0x00,
}