//! Wire fixtures from the official 26.2 / 26.3 packet codecs.
use crate::packet_processors::process_decode;
use crate::packet_utils::Buf;
use crate::states::{config, login, play};
use crate::{protocol, Bot, Compression, ProtocolState, Stream};
use libdeflater::{CompressionLvl, Compressor, Decompressor};
use mio::Token;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn bytes(buf: Buf) -> Vec<u8> {
    buf.buffer[..buf.get_writer_index() as usize].to_vec()
}

fn connection(state: ProtocolState) -> (Bot, TcpStream, Compression) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let stream = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
    let (peer, _) = listener.accept().unwrap();
    peer.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
    let bot = Bot {
        token: Token(0),
        stream: Stream::TCP(mio::net::TcpStream::from_std(stream)),
        name: "Bot_0".into(),
        id: 0,
        entity_id: 123,
        compression_threshold: -1,
        state,
        kicked: false,
        teleported: false,
        input_flags: 0,
        x: 10.0,
        y: 20.0,
        z: 30.0,
        yaw: 0.0,
        pitch: 0.0,
        buffering_buf: Buf::new(),
        joined: true,
        chat_interval_ticks: 20,
        reconnect_at: None,
        auth_sent: false,
        auth_enabled: false,
    };
    let compression = Compression {
        compressor: Compressor::new(CompressionLvl::default()),
        decompressor: Decompressor::new(),
    };
    (bot, peer, compression)
}

#[test]
fn handshake_advertises_the_selected_release() {
    let handshake =
        login::write_handshake_packet(protocol::PROTOCOL_VERSION, "localhost".into(), 25565, 2);
    let mut expected = vec![
        0,
        if cfg!(feature = "mc-26-2") {
            0x88
        } else {
            0x89
        },
        6,
        9,
    ];
    expected.extend_from_slice(b"localhost");
    expected.extend_from_slice(&[0x63, 0xDD, 2]);
    assert_eq!(bytes(handshake), expected);
}

#[test]
fn version_specific_actions_match_wire_fixtures() {
    if cfg!(feature = "mc-26-2") {
        assert_eq!(bytes(play::write_animation(false)), [0x3F, 0]);
        assert_eq!(bytes(play::write_animation(true)), [0x3F, 1]);
        assert_eq!(bytes(play::write_held_slot(8)), [0x35, 0, 8]);
    } else {
        assert_eq!(bytes(play::write_animation(false)), [0x2E]);
        assert_eq!(bytes(play::write_animation(true)), [0x2E]);
        assert_eq!(bytes(play::write_held_slot(8)), [0x36, 0, 8]);
    }
    assert_eq!(bytes(play::write_player_input(0x20)), [0x2B, 0x20]);
    assert_eq!(bytes(play::write_tick_end()), [0x0D]);
    assert_eq!(
        bytes(play::write_entity_action(123, 1, 0)),
        [0x2A, 123, 1, 0]
    );
    assert_eq!(
        bytes(play::write_keep_alive_packet(1)),
        [0x1C, 0, 0, 0, 0, 0, 0, 0, 1]
    );
    let position = bytes(play::write_pos(0.0, 0.0, 0.0, 0.0, 0.0));
    assert_eq!(position.len(), 34);
    assert_eq!(position[0], 0x1F);
    assert!(position[1..].iter().all(|b| *b == 0));
}

#[test]
fn teleport_reads_velocity_and_full_relative_flags() {
    let (mut bot, mut peer, mut compression) = connection(ProtocolState::Play);
    let mut packet = Buf::new();
    packet.write_var_u32(if cfg!(feature = "mc-26-2") {
        0x48
    } else {
        0x49
    });
    packet.write_var_u32(42);
    for value in [1.0, 2.0, 3.0, 111.0, 222.0, 333.0] {
        packet.write_f64(value);
    }
    packet.write_f32(90.0);
    packet.write_f32(45.0);
    packet.write_u32(0x10D); // Relative X/Z, yaw, rotate delta; absolute Y.
    assert_eq!(
        process_decode(&mut packet, &mut bot, &mut compression),
        Some(())
    );
    assert_eq!((bot.x, bot.y, bot.z), (11.0, 2.0, 33.0));
    assert!(bot.teleported);
    assert_eq!(packet.get_reader_index(), packet.get_writer_index());
    let mut expected = vec![if cfg!(feature = "mc-26-2") { 2 } else { 34 }, 0, 42];
    if !cfg!(feature = "mc-26-2") {
        for value in [11.0_f64, 2.0, 33.0] {
            expected.extend_from_slice(&value.to_be_bytes());
        }
        for value in [90.0_f32, 45.0] {
            expected.extend_from_slice(&value.to_be_bytes());
        }
    }
    expected.extend_from_slice(&[1, 0x2C]); // Player Loaded.
    let mut response = vec![0; expected.len()];
    peer.read_exact(&mut response).unwrap();
    assert_eq!(response, expected);
}

#[test]
fn play_cookies_use_the_play_response_id() {
    let (mut bot, mut peer, mut compression) = connection(ProtocolState::Play);
    let mut packet = Buf::new();
    packet.write_var_u32(0x15);
    packet.write_sized_str("test:key");
    assert_eq!(
        process_decode(&mut packet, &mut bot, &mut compression),
        Some(())
    );
    let mut response = [0; 12];
    peer.read_exact(&mut response).unwrap();
    assert_eq!(&response, b"\x0b\x15\x08test:key\x00");
}

#[test]
fn configuration_selects_the_correct_known_packs_packet() {
    let (mut bot, mut peer, mut compression) = connection(ProtocolState::Config);
    let mut packet = Buf::new();
    packet.write_var_u32(if cfg!(feature = "mc-26-2") {
        0x0E
    } else {
        0x0F
    });
    assert_eq!(
        process_decode(&mut packet, &mut bot, &mut compression),
        Some(())
    );
    let mut response = [0; 3];
    peer.read_exact(&mut response).unwrap();
    assert_eq!(response, [2, 7, 0]);
    assert_eq!(bytes(config::write_acknowledge_configuration()), [3]);
}

#[test]
fn transfer_packet_id_is_a_multibyte_varint() {
    let (mut bot, _peer, mut compression) = connection(ProtocolState::Play);
    let mut packet = Buf::new();
    packet.write_var_u32(if cfg!(feature = "mc-26-2") {
        0x81
    } else {
        0x84
    });
    packet.write_sized_str("localhost");
    packet.write_u16(25565);
    assert_eq!(
        process_decode(&mut packet, &mut bot, &mut compression),
        Some(())
    );
    assert!(bot.kicked); // Preserve the fork's unsupported-transfer behavior.
}

#[test]
fn unsigned_chat_uses_current_timestamp_and_last_seen_update() {
    let before = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let mut packet = play::write_chat_message("Wow");
    assert_eq!(packet.read_var_u32().0, 0x09);
    assert_eq!(packet.read_sized_string(), "Wow");
    assert!(packet.read_u64() >= before);
    assert_eq!(packet.read_u64(), 0); // Salt.
    assert!(!packet.read_bool()); // Unsigned offline-mode message.
    assert_eq!(packet.read_var_u32().0, 0); // Last-seen offset.
    assert_eq!(packet.read_bytes(3), [0, 0, 0]); // 20-bit acknowledgement set.
    assert_eq!(packet.read_byte(), 0); // Ignore checksum.
    assert_eq!(packet.get_reader_index(), packet.get_writer_index());
}
