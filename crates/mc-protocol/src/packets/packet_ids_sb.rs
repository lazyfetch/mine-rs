use num_enum::TryFromPrimitive;

use crate::packets::{utils::Packet};
 
// -- Handshake --
#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum HandshakeServerboundPacketId {
    Handshake = 0x00,
}

pub struct Handshake;

impl Packet for Handshake {
    type Id = HandshakeServerboundPacketId;
    const ID: Self::Id = HandshakeServerboundPacketId::Handshake;
}

// -- Handshake end --

// -- Login --
#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum LoginServerboundPacketId {
    Login = 0x00,
    EncryptionResponse = 0x01,
    LoginPlugin = 0x02,
    LoginAcknowledged = 0x03,
}

pub struct Login;

impl Packet for Login {
    type Id = LoginServerboundPacketId;
    const ID: Self::Id = LoginServerboundPacketId::Login;
}

pub struct LoginAcknowledged;

impl Packet for LoginAcknowledged {
    type Id = LoginServerboundPacketId;
    const ID: Self::Id = LoginServerboundPacketId::LoginAcknowledged;
}

// -- Login end --

// -- Configure --
#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum ConfigureServerboundPacketId {
    ClientInformation = 0x00,
    CookieResponse = 0x01,
    PluginMessage = 0x02,
    AcknowledgeFinishConfiguration = 0x03,
    KeepAlive = 0x04,
    Pong = 0x05,
    ResourcePackResponce = 0x06,
    KnownPacks = 0x07,
    CustomClickAction = 0x08,
}

// -- Configure end

// -- Play -- 
#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(i32)]
pub enum PlayServerboundPacketId {
    ConfirmTeleportation = 0x00,
    QueryBlockEntityTag = 0x01,
    BundleItemSelected = 0x02,
    ChangeDifficulty = 0x03,
    ChangeGameMode = 0x04,
    AcknowledgeMessage = 0x05,
    ChatCommand = 0x06,
    SignedChatCommand = 0x07,
    ChatMessage = 0x08,
    ChunkBatchReceived = 0x0A,
    ClientStatus = 0x0B,
    ClientTickEnd = 0x0C,
    ClientInformation = 0x0D,
    CommandSuggestionsRequest = 0x0E,
    AcknowledgeConfiguration = 0x0F,
    ClickContainerButton = 0x10,
    ClickContainer = 0x11,
    CloseContainer = 0x12,
    ChangeContainerSlotSlate = 0x13,
    CookieResponse = 0x14,
    PluginMessage = 0x15,
    DebugSampleSubscription = 0x16,
    EditBook = 0x17,
    QueryEntityTag = 0x18,
    Interact = 0x19,
    JigsawGenerate = 0x1A,
    KeepAlive = 0x1B,
    LockDifficulty = 0x1C,
    SetPlayerPosition = 0x1D,
    SetPlayerPositionAndRotation = 0x1E,
    SetPlayerRotation = 0x1F,
    SetPlayerMovementFlags = 0x20,
    MoveVehicle = 0x21,
    PaddleBoat = 0x22,
    PickItemFromBlock = 0x23,
    PickItemFromEntity = 0x24,
    PingRequest = 0x25,
    PlaceRecipe = 0x26,
    PlayerAbilities = 0x27,
    PlayerAction = 0x28,
    PlayerCommand = 0x29,
    PlayerInput = 0x2A,
    PlayerLoaded = 0x2B,
    Pong = 0x2C,
    ChangeRecipeBookSettings = 0x2D,
    SetSeenRecipe = 0x2E,
    RenameItem = 0x2F,
    ResourcePackResponse = 0x30,
    SeenAdvancements = 0x31,
    SelectTrade = 0x32,
    SetBeaconEffect = 0x33,
    SetHeldItem = 0x34,
    ProgramCommandBlock = 0x35,
    ProgramCommandBlockMinecart = 0x36,
    SetCreativeModeSlot = 0x37,
    ProgramJigsawBlock = 0x38,
    ProgramStructureBlock = 0x39,
    SetTestBlock = 0x3A,
    UpdateSign = 0x3B,
    SwingArm = 0x3C,
    TeleportToEntity = 0x3D,
    TestInstanceBlockAction = 0x3E,
    UseItemOn = 0x3F,
    UseItem = 0x40,
    CustomClickAction = 0x41, 
}

pub struct KeepAlivePlay;

impl Packet for KeepAlivePlay {
    type Id = PlayServerboundPacketId;
    const ID: Self::Id = PlayServerboundPacketId::KeepAlive;
}

// -- Play end