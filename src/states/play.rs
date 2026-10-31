use crate::packet_utils::Buf;
use crate::protocol;
use crate::{Bot, Compression};

/// Clientbound Keep Alive (play)
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Keep_Alive_(play)
pub fn process_keep_alive_packet(buffer: &mut Buf, bot: &mut Bot, compression: &mut Compression) {
    bot.send_packet(write_keep_alive_packet(buffer.read_u64()), compression);
}

/// Disconnect (login/config/play)
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Disconnect_(login)
pub fn process_kick(buffer: &mut Buf, bot: &mut Bot, _compression: &mut Compression) {
    println!("bot was kicked for \"{}\"", buffer.read_sized_string());
    bot.kicked = true;
}

/// Login (play)
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_(play)
pub fn process_join_game(buffer: &mut Buf, bot: &mut Bot, _compression: &mut Compression) {
    bot.entity_id = buffer.read_u32();
}

/// Synchronize Player Position
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Player_Position
pub fn process_teleport(buffer: &mut Buf, bot: &mut Bot, compression: &mut Compression) {
    let id = buffer.read_var_u32().0;
    let x = buffer.read_f64();
    let y = buffer.read_f64();
    let z = buffer.read_f64();
    if !protocol::IS_1_21_11 {
        let _delta_x = buffer.read_f64();
        let _delta_y = buffer.read_f64();
        let _delta_z = buffer.read_f64();
    }
    let yaw = buffer.read_f32();
    let pitch = buffer.read_f32();
    let flags = if protocol::IS_1_21_11 {
        buffer.read_byte() as u32
    } else {
        buffer.read_u32()
    };
    if flags & 0b00001 == 0 {
        bot.x = x;
    } else {
        bot.x += x;
    }
    if flags & 0b00010 == 0 {
        bot.y = y;
    } else {
        bot.y += y;
    }
    if flags & 0b00100 == 0 {
        bot.z = z;
    } else {
        bot.z += z;
    }
    bot.yaw = yaw + if flags & 0x08 != 0 { bot.yaw } else { 0.0 };
    bot.pitch = (pitch + if flags & 0x10 != 0 { bot.pitch } else { 0.0 }).clamp(-90.0, 90.0);
    bot.send_packet(
        write_tele_confirm(id, bot.x, bot.y, bot.z, bot.yaw, bot.pitch),
        compression,
    );
    if !bot.teleported {
        let mut loaded = Buf::new();
        loaded.write_packet_id(protocol::PLAY_LOADED); // Player Loaded
        bot.send_packet(loaded, compression);
    }
    bot.teleported = true;
    println!("{x}, {y}, {z}");
}

/// Chat Message
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Message
pub fn write_chat_message(message: &str) -> Buf {
    // ClientChatMessagePacket
    let mut buf = Buf::new();
    buf.write_packet_id(if protocol::IS_1_21_11 { 0x08 } else { 0x09 });

    buf.write_sized_str(message);

    // 1.19 signing fields
    buf.write_u64(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
    ); // timestamp
    buf.write_u64(0); // salt
    buf.write_bool(false); // has signature
    buf.write_var_u32(0); // count
    buf.write_bytes(&[0; 3]); // bitset
    buf.write_u8(0); // last-seen checksum (0 skips checksum verification)

    buf
}

/// Swing Arm
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Swing_Arm
pub fn write_animation(off_hand: bool) -> Buf {
    // ClientAnimationPacket
    let mut buf = Buf::new();
    buf.write_packet_id(protocol::PLAY_ANIMATION);
    // 26.3 replaced Swing Arm with the empty Punch packet.
    if protocol::IS_26_2 || protocol::IS_1_21_11 {
        buf.write_var_u32(if off_hand { 1 } else { 0 });
    }

    buf
}

/// Player Action (serverbound)
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Command
pub fn write_entity_action(entity_id: u32, action_id: u32, jump_boost: u32) -> Buf {
    // ClientEntityActionPacket
    let mut buf = Buf::new();
    buf.write_packet_id(protocol::PLAY_ENTITY_ACTION);

    buf.write_var_u32(entity_id);
    buf.write_var_u32(action_id);
    buf.write_var_u32(jump_boost);

    buf
}

/// Set Held Item (serverbound)
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Held_Item_(serverbound)
pub fn write_held_slot(slot: u16) -> Buf {
    // ClientHeldItemChangePacket
    let mut buf = Buf::new();
    buf.write_packet_id(protocol::PLAY_HELD_SLOT);

    buf.write_u16(slot);

    buf
}

/// Confirm Teleportation
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Confirm_Teleportation
pub fn write_tele_confirm(id: u32, x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Buf {
    // ClientTeleportConfirmPacket
    let mut buf = Buf::new();
    buf.write_packet_id(0x00);

    buf.write_var_u32(id);
    if !protocol::IS_26_2 && !protocol::IS_1_21_11 {
        buf.write_f64(x);
        buf.write_f64(y);
        buf.write_f64(z);
        buf.write_f32(yaw);
        buf.write_f32(pitch);
    }

    buf
}

/// Serverbound Keep Alive (play)
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Keep_Alive_(play)
pub fn write_keep_alive_packet(id: u64) -> Buf {
    // ClientKeepAlivePacket
    let mut buf = Buf::new();
    buf.write_packet_id(protocol::PLAY_KEEP_ALIVE_SERVERBOUND);

    buf.write_u64(id);

    buf
}

pub fn write_current_pos(bot: &Bot) -> Buf {
    write_pos(bot.x, bot.y, bot.z, 0.0, 0.0)
}

/// Set Player Position and Rotation
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position_and_Rotation
pub fn write_pos(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Buf {
    // ClientPlayerPositionAndRotationPacket
    let mut buf = Buf::new();
    buf.write_packet_id(protocol::PLAY_POSITION_SERVERBOUND);

    buf.write_f64(x);
    buf.write_f64(y);
    buf.write_f64(z);

    buf.write_f32(yaw);
    buf.write_f32(pitch);

    buf.write_bool(false);

    buf
}

/// Player Input: shift is bit 5, sprint is bit 6.
pub fn write_player_input(flags: u8) -> Buf {
    let mut buf = Buf::new();
    buf.write_packet_id(protocol::PLAY_INPUT);
    buf.write_u8(flags);
    buf
}

/// Cookie Response in play uses a different ID from configuration.
pub fn process_cookie_request_packet(
    buffer: &mut Buf,
    bot: &mut Bot,
    compression: &mut Compression,
) {
    let identifier = buffer.read_sized_string();
    let mut buf = Buf::new();
    buf.write_packet_id(0x15);
    buf.write_sized_str(identifier);
    buf.write_bool(false);
    bot.send_packet(buf, compression);
}

/// 26.3 permits one position update per client tick, delimited by this packet.
pub fn write_tick_end() -> Buf {
    let mut buf = Buf::new();
    buf.write_packet_id(protocol::PLAY_TICK_END);
    buf
}
