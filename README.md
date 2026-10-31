# `rust-mc-bot`

A high performance stress testing tool for custom Minecraft server implementations

## Motivation

When I was working on the [Minestom Project](https://github.com/Minestom/Minestom) we needed a reliable way to verify the end-to-end performance of our code but all the existing bot implementations would crash before the server did. So I made a bot implementation dedicated to having the best performance possible (and least unnecessary features).

## Usage

1. Clone the code
    - If you want to stress test a server implementing an older version of the minecraft protocol the corresponding tag
    - ```bash
      git checkout tags/1.18.1
      ```
2. Compile the code
    - This port targets **Minecraft Java 26.3** (protocol 777) by default. For **26.2** (protocol 776), build with `cargo build --release --features mc-26-2` instead. Choose the matching build for your server; version selection happens at compile time and the command-line arguments are unchanged.
    - Make sure Rust is installed. See [here](https://rustup.rs).
    - ```bash
      cargo build --release
      ```
    - Executable will be built to `target/release/rust-mc-bot` (linux/macos) or `target/release/rust-mc-bot.exe` (Windows)
3. Start the bots
    - Keep `usernames.txt` and `messages.txt` in the current directory when launching the program. Each bot receives a random username from `usernames.txt`; repeated names receive `_1`, `_2`, and so on. Chat messages are selected randomly from `messages.txt`, one message per line.
    - Usage:
      ```bash
      ./rust-mc-bot <ip:port or path> <count> [threads]
      ./rust-mc-bot 127.0.0.1:25565 1000
      ```
    - Alternative:
      ```bash
      cargo run --release -- <ip:port or path> <count> [threads]
      cargo run --release -- 127.0.0.1:25565 1000
      # Minecraft Java 26.2:
      cargo run --release --features mc-26-2 -- 127.0.0.1:25565 1000
      ```

## Port scope

The join rate, movement, random chat/actions, threading, Unix socket support and offline-mode requirement are preserved. Only protocol compatibility is updated. Minecraft 26.3 replaces the hand-specific Swing Arm packet with Punch, so its animation no longer selects a hand. Sneaking uses Player Input in both versions; 26.3 also needs position/rotation in teleport confirmations and a Client Tick End packet after movement.

Packet IDs and layouts were checked against the official Mojang 26.2 and 26.3 server JARs (`version.json`, `GameProtocols`, `ConfigurationProtocols` and packet codecs). Run the protocol regression tests for both builds with:

```bash
cargo test --locked
cargo test --locked --features mc-26-2
```

## Known Issues

Using `localhost` as the IP on machines with ipv6 may cause the bots to not connect to the server. Please use `127.0.0.1` instead.

Currently, the bots do not support online mode to prevent abuse and to improve performance.

The original random chat behavior is preserved, so vanilla servers can kick bots for chat spam. The local protocol smoke tests use operator bots to avoid this server-side limit.

## Disclaimer

This should **ONLY** be used test your own server. We do not endorse the use of this for any other purposes than testing your own infrastructure.

Please be aware that attempting to execute this with an external server as a target can be seen as **illegal** as it simulates a layer 7 DoS (denial-of-service) attack, which is against the law in most countries.

