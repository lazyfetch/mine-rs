<!-- markdownlint-configure-file {
  "MD013": {
    "code_blocks": false,
    "tables": false
  },
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# mine-rs

Lightweight Implementation of the Minecraft client protocol in Rust.

</div>

<p align="center">
  <a href="#"><img src="https://img.shields.io/badge/rust-1.89+-orange.svg" alt="Rust Version"></a>
  <a href="https://github.com/YOUR_USERNAME/mine-rs/blob/main/LICENSE"><img src="https://img.shields.io/github/license/YOUR_USERNAME/mine-rs" alt="License"></a>
  <a href="https://github.com/YOUR_USERNAME/mine-rs/actions"><img src="https://img.shields.io/github/actions/workflow/status/YOUR_USERNAME/mine-rs/rust.yml?branch=main" alt="Build Status"></a>
</p>

`mine-rs` is an asynchronous, lightweight implementation of the Minecraft client protocol, crafted in Rust. The primary goal of this project is to explore protocol implementation details and experiment with a clean, event-driven architecture using modern Rust features.

> **⚠️ Project Status: Proof of Concept**
>
> Please be aware that `mine-rs` is a personal project created for learning and experimentation purposes. It is currently in a proof-of-concept stage and is not a complete, production-ready client. While the core architecture is in place, many features are likely missing or incomplete.
>
> The project is not under active development, but feel free to explore the code, learn from the patterns used, or **fork the repository** to build your own implementation!

## ✨ Features

- **Fluent Builder API:** A clean, chainable builder pattern allows for intuitive and readable client configuration.
- **📦 Registry-Based Architecture:** State management is decoupled into specialized registries (e.g., `EntityRegistry`, `PlayerRegistry`), which provides clear separation of concerns.
- **🚀 Asynchronous & Event-Driven:** Built on top of `tokio`, the client operates asynchronously. Game events are exposed through simple, closure-based handlers (e.g., `on_spawn`, `on_move`), making it easy to react to server updates.
- **🎯 Lightweight & Focused:** Aims to provide a minimal-dependency foundation for building Minecraft bots, tools, or custom clients.

## 🚀 Getting Started

Here is a basic example of how to configure a client, register event handlers, and initiate a connection.

```rust
// main.rs
use mine_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure the client using the fluent builder pattern.
    let mut client = Client::build()
        .with_host("localhost")
        .with_port(25565)
        .with_username("Explorer_18")
        .finish();

    // 2. Register event handlers using closures for clean, inline logic.
    // Entity events are handled through the `entities` registry.
    client.entities
        .on_spawn(|entity| {
            println!("[Event] Entity spawned: {:?}", entity);
        })
        .on_move(|entity_move_info| {
            println!("[Event] Entity moved: ID {}", entity_move_info.id);
        })
        .on_despawn(|entity_id| {
            println!("[Event] Entity despawned: {}", entity_id);
        });

    // Player-specific events are handled through the `player` registry.
    client.player.on_synchronize(|player_state| {
        println!("[Event] Player state synchronized: {:?}", player_state);
    });

    // 3. Connect to the server and get the connection handle.
    // The handle is responsible for the network read/write loop.
    let connection_handle = client.connect().await?;

    // The handle can now be used to send packets to the server.
    // For example (hypothetical API):
    // handle.player.move(x, y, z);

    // In a real application, you would keep the handle alive to maintain the connection.
    // Here we'll just wait indefinitely.
    std::future::pending::<()>().await;

    Ok(())
}
```

## 🏗️ Architecture Overview

The design of `mine-rs` emphasizes modularity and ease of use.

#### Registries
The core of the client's state management revolves around **Registries**. Instead of a monolithic `Client` struct holding all state, data is segregated into logical units:
-   **`EntityRegistry`**: Tracks all spawned entities in the world, their positions, and metadata.
-   **`PlayerRegistry`**: Manages the local player's state, such as health, position, and inventory.
-   **(Other Registries)**: This pattern can be extended for things like `WorldRegistry`, `InventoryRegistry`, etc.

Each registry exposes methods to register event handlers (`.on_spawn`, `.on_synchronize`, etc.), which are triggered internally by the packet processing loop.

#### Network Layer
The connection and packet handling are managed by an asynchronous task spawned by the `client.connect()` method. This task is responsible for:
1.  Reading raw byte streams from the TCP socket.
2.  Parsing them into structured `Packet` types.
3.  Dispatching these packets to the relevant registries for state updates and event emissions.
4.  Sending outbound packets to the server.

This decouples the networking logic from the client's state and API, making the codebase cleaner and easier to maintain.

#### `Macro_rules!`, types and abstractions for make your own clientbound handling!

