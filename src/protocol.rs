// Minecraft Java release protocols, verified against the official server JARs.
// Packet IDs include the clientbound bundle delimiter at play ID 0x00.
#[cfg(feature = "mc-26-2")]
pub const PROTOCOL_VERSION: u32 = 776;
#[cfg(not(feature = "mc-26-2"))]
pub const PROTOCOL_VERSION: u32 = 777;

pub const IS_26_2: bool = cfg!(feature = "mc-26-2");

pub const CONFIG_TRANSFER: u32 = if IS_26_2 { 0x0B } else { 0x0C };
pub const CONFIG_KNOWN_PACKS: u32 = if IS_26_2 { 0x0E } else { 0x0F };
pub const CONFIG_CODE_OF_CONDUCT: u32 = if IS_26_2 { 0x13 } else { 0x14 };

pub const PLAY_COOKIE_REQUEST: u32 = 0x15;
pub const PLAY_DISCONNECT: u32 = 0x20;
pub const PLAY_KEEP_ALIVE: u32 = if IS_26_2 { 0x2C } else { 0x2D };
pub const PLAY_LOGIN: u32 = if IS_26_2 { 0x31 } else { 0x32 };
pub const PLAY_POSITION: u32 = if IS_26_2 { 0x48 } else { 0x49 };
pub const PLAY_TRANSFER: u32 = if IS_26_2 { 0x81 } else { 0x84 };
pub const PLAY_HELD_SLOT: u32 = if IS_26_2 { 0x35 } else { 0x36 };
pub const PLAY_ANIMATION: u32 = if IS_26_2 { 0x3F } else { 0x2E };
