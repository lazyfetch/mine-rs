// use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, Hash, /*TryFromPrimitive*/)]
#[repr(i32)]
pub enum LoginClientboundPacketId {
    Disconnect = 0x00,
    EncryptionRequest = 0x01,
    LoginSuccess = 0x02,
    SetCompression = 0x03,
    LoginPluginRequest = 0x04,
    CookieRequest = 0x05,
}

#[derive(Debug, Eq, PartialEq, Hash, /*TryFromPrimitive*/)]
#[repr(i32)]
pub enum ConfigureClientboundPacketId {
    CookieRequest = 0x00,
    PluginMessage = 0x01,
    Disconnect = 0x02,
    FinishConfiguration = 0x03,
    KeepAlive = 0x04,
    Ping = 0x05,
    ResetChat = 0x06,
    RegistryData = 0x07,
    RemoveResourcePack = 0x08,
    AddResourcePack = 0x09,
    StoreCookie = 0x0A,
    Transfer = 0x0B,
    FeatureFlags = 0x0C,
    UpdateTags = 0x0D,
    KnownPacks = 0x0E,
    CustomReportDetails = 0x0F,
    ServerLinks = 0x10,
    ClearDialog = 0x11,
    ShowDialog = 0x12,
}

#[derive(Debug, Eq, PartialEq, Hash, /*TryFromPrimitive*/)]
#[repr(i32)]
pub enum PlayClientboundPacketId {
    SpawnEntity = 0x00,
    SpawnPlayer = 0x04,
    EntityAnimation = 0x06,
    KeepAlive = 0x21,
    ChatMessage = 0x0F,
}