// Minecraft Java release protocols, verified against the official server JARs.
// Packet IDs include the clientbound bundle delimiter at play ID 0x00.
#[cfg(feature = "mc-1-21-11")]
pub const PROTOCOL_VERSION: u32 = 774;
#[cfg(all(not(feature = "mc-1-21-11"), feature = "mc-26-2"))]
pub const PROTOCOL_VERSION: u32 = 776;
#[cfg(all(not(feature = "mc-1-21-11"), not(feature = "mc-26-2")))]
pub const PROTOCOL_VERSION: u32 = 777;

pub const IS_26_2: bool = cfg!(feature = "mc-26-2");
pub const IS_1_21_11: bool = cfg!(feature = "mc-1-21-11");

pub const CONFIG_TRANSFER: u32 = if IS_26_2 { 0x0B } else { 0x0C };
pub const CONFIG_KNOWN_PACKS: u32 = if IS_26_2 { 0x0E } else { 0x0F };
pub const CONFIG_CODE_OF_CONDUCT: u32 = if IS_26_2 { 0x13 } else { 0x14 };

pub const PLAY_COOKIE_REQUEST: u32 = 0x15;
pub const PLAY_DISCONNECT: u32 = if IS_1_21_11 { 0x1C } else { 0x20 };
pub const PLAY_KEEP_ALIVE: u32 = if IS_1_21_11 {
    0x2B
} else if IS_26_2 {
    0x2C
} else {
    0x2D
};
pub const PLAY_LOGIN: u32 = if IS_1_21_11 {
    0x30
} else if IS_26_2 {
    0x31
} else {
    0x32
};
pub const PLAY_POSITION: u32 = if IS_1_21_11 {
    0x46
} else if IS_26_2 {
    0x48
} else {
    0x49
};
pub const PLAY_TRANSFER: u32 = if IS_1_21_11 {
    0x7A
} else if IS_26_2 {
    0x81
} else {
    0x84
};
pub const PLAY_HELD_SLOT: u32 = if IS_1_21_11 {
    0x34
} else if IS_26_2 {
    0x35
} else {
    0x36
};
pub const PLAY_ANIMATION: u32 = if IS_1_21_11 {
    0x3C
} else if IS_26_2 {
    0x3F
} else {
    0x2E
};
pub const PLAY_KEEP_ALIVE_SERVERBOUND: u32 = if IS_1_21_11 { 0x1B } else { 0x1C };
pub const PLAY_POSITION_SERVERBOUND: u32 = if IS_1_21_11 { 0x1E } else { 0x1F };
pub const PLAY_ENTITY_ACTION: u32 = if IS_1_21_11 { 0x29 } else { 0x2A };
pub const PLAY_INPUT: u32 = if IS_1_21_11 { 0x2A } else { 0x2B };
pub const PLAY_TICK_END: u32 = if IS_1_21_11 { 0x0C } else { 0x0D };
pub const PLAY_LOADED: u32 = if IS_1_21_11 { 0x2B } else { 0x2C };
