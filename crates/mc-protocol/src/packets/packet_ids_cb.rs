use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum LoginClientboundPacketId {
    Disconnect = 0x00,
    EncryptionRequest = 0x01,
    LoginSuccess = 0x02,
    SetCompression = 0x03,
    LoginPluginRequest = 0x04,
    CookieRequest = 0x05,
}

#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
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

#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum PlayClientboundPacketId {
    BundleDelimiter = 0x00,
    SpawnEntity = 0x01,
    EntityAnimation = 0x02,
    AwardStatistics = 0x03,
    AcknowledgeBlockChange = 0x04,
    SetBlockDestroyStage = 0x05,
    BlockEntityData = 0x06,
    BlockAction = 0x07,
    BlockUpdate = 0x08,
    BossBar = 0x09,
    ChangeDifficulty = 0x0A,
    ChunkBatchFinished = 0x0B,
    ChunkBatchStart = 0x0C,
    ChunkBiomes = 0x0D,
    Commands = 0x10,
    CloseContainer = 0x11,
    SetContainerContent = 0x12,
    SetContainerProperty = 0x13,
    SetContainerSlot = 0x14,
    CookieRequest = 0x015,
    // ...
    KeepAlive = 0x26,
    UpdateEntityPosition = 0x2E,
    UpdateEntityPositionAndRotation = 0x2F,
    UpdateEntityRotation = 0x30,
}