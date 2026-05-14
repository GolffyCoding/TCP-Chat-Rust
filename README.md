<p align="center">
  <h1 align="center">🗯️ TCP Chat</h1>
  <p align="center">A blazing-fast, asynchronous TCP chat server built with Rust</p>
</p>

<p align="center">
  <a href="https://github.com/yourusername/tcp_chat"><img src="https://img.shields.io/github/stars/yourusername/tcp_chat?style=social" alt="Stars"></a>
  <a href="https://github.com/yourusername/tcp_chat"><img src="https://img.shields.io/github/forks/yourusername/tcp_chat?style=social" alt="Forks"></a>
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="License">
  <img src="https://img.shields.io/badge/rust-1.92%2B-orange" alt="Rust 1.92+">
  <img src="https://img.shields.io/badge/build-passing-brightgreen" alt="Build">
</p>

## 📚 Overview

**TCP Chat** is a high-performance, asynchronous chat server implemented in Rust. Built on the Tokio runtime, it provides a robust, concurrent messaging platform that scales to thousands of connections with minimal overhead.

### ✨ Features

- 🚀 **High Performance** - Async I/O with Tokio runtime
- 👥 **Multi-client Support** - Handle up to 10,000 concurrent connections
- 💬 **Real-time Broadcasting** - Messages broadcast to all connected clients
- 🔧 **Rich Commands** - Built-in client commands for enhanced experience
- ⚙️ **Configurable** - Customizable host, port, limits, and rate limiting
- 📊 **Structured Logging** - JSON-formatted tracing logs for observability
- 🛡️ **Graceful Shutdown** - Clean client cleanup on disconnect

## 🏗️ Architecture

### Threat Model

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          THREAT MODEL                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│ External Attacker                                                           │
│   ├─ DoS/Slowloris (slow reads, connection exhaustion)                      │
│   ├─ Memory exhaustion (unbounded messages, large payloads)                 │
│   ├─ Channel flooding (message spam)                                          │
│   ├─ Malformed protocol (invalid UTF-8, null bytes)                         │
│   ├─ Username injection (control chars, newlines)                           │
│   └─ Resource exhaustion (CPU, file descriptors)                            │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                        TRUST BOUNDARIES                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│ NETWORK LAYER (UNTRUSTED) → PROTOCOL LAYER (VALIDATED) → STATE (TRUSTED)  │
│         │                            │                    │                │
│    Raw TCP bytes              Sanitized strings         Protected state    │
│    Client IP                 Validated commands        Rate-limited access│
└─────────────────────────────────────────────────────────────────────────────┘
```

### Attack Surfaces & Mitigations

| Attack Vector | Mitigation | Implementation |
|--------------|------------|----------------|
| Unbounded messages | Message size limits | `max_message_size` config, validation in parser |
| Username injection | Character sanitization | `InputSanitizer::sanitize_username()` |
| Connection exhaustion | Connection limits | `max_connections` config, semaphore pattern |
| Slowloris | Idle timeouts | `idle_timeout_secs`, tokio::time::timeout |
| Memory exhaustion | Bounded channels | `broadcast::channel(N)` with panic on overflow |
| UTF-8 crashes | Validation | `std::str::from_utf8()` with error handling |

### Async Event Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        ASYNC EVENT FLOW                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Accept               Read Line              Process                       │
│    │                    │                     │                            │
│    ▼                    ▼                     ▼                            │
│ ┌───────┐          ┌──────────┐        ┌───────────┐                       │
│ │TcpList│──wait────▶│BufReader │──rate──▶│Command    │                       │
│ └───────┘    accept│└──────────┘  limit  │Handler    │                       │
│                                           └───────────┘                       │
│                                                │                              │
│                                                ▼                              │
│                                        ┌──────────────┐                       │
│                                        │Message Bus   │                       │
│                                        │(bounded)     │                       │
│                                        └──────────────┘                       │
│                                                │                              │
│                                                ▼                              │
│                                       ┌────────────────┐                     │
│                                       │Broadcast Task  │                     │
│                                       └────────────────┘                     │
│                                                │                              │
│                                                ▼                              │
│                                    ┌─────────────────────┐                    │
│                                    │Client Writers       │                    │
│                                    └─────────────────────┘                    │
└───────────────────────────────────────────────────────────────────────────────┘
```

### Client Lifecycle

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Connect   │────▶│   Handshake │────▶│   Active    │────▶│   Cleanup   │
│  (TcpStream)│     │(username set)│     │(read/write) │     │ (remove from│
└─────────────┘     └─────────────┘     └─────────────┘     │  registry)  │
                          │              │                └─────────────┘
                          │              │                       │
                          ▼              ▼                       ▼
                     [JOIN MSG]   [RATE LIMIT]             [LEAVE MSG]
```

### Isolation Strategy

- **Client Isolation**: Each client runs in its own spawned task
- **State Isolation**: Shared state via `Arc<DashMap>` (lock-free reads)
- **Panic Containment**: `tokio::spawn` isolates panics to individual tasks
- **Resource Limits**: Bounded channels prevent memory exhaustion

## 📁 Project Structure

```
src/
├── main.rs              # Entry point, graceful shutdown
├── lib.rs               # Public API exports
├── config/
│   └── mod.rs           # Server configuration (immutable)
├── security/
│   ├── mod.rs           # Security module exports
│   ├── rate_limiter.rs  # Token bucket rate limiting
│   ├── sanitizer.rs     # Input validation and sanitization
│   └── validator.rs     # Protocol validation
├── networking/
│   ├── mod.rs           # Networking abstractions
│   └── listener.rs      # TCP listener with backlog control
├── protocol/
│   ├── mod.rs           # Protocol layer
│   ├── parser.rs        # Line-based protocol parser
│   └── message.rs       # Message types and constants
├── session/
│   ├── mod.rs           # Session management
│   ├── handler.rs       # Client session handler
│   └── timeout.rs       # Idle/connection timeouts
├── commands/
│   ├── mod.rs           # Command processing
│   ├── registry.rs      # Command registry
│   └── handlers/
│       ├── name.rs      # /name command
│       ├── quit.rs      # /quit command
│       └── users.rs     # /users command
├── state/
│   └── mod.rs           # Shared state management (DashMap)
├── message_bus/
│   ├── mod.rs           # Message bus abstraction
│   └── broadcaster.rs   # Broadcast implementation
└── logging/
    └── mod.rs           # Structured logging setup
```

### Why Modularization Improves Security

1. **Separation of Concerns**: Security logic is isolated from business logic
2. **Auditability**: Each module can be reviewed independently for vulnerabilities
3. **Testability**: Security functions can be unit tested in isolation
4. **Maintainability**: Changes to one layer don't affect others

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.92 or higher
- **Cargo** (comes with Rust)

### Installation

```bash
# Clone the repository
git clone https://github.com/tcp_chat.git
cd tcp_chat

# Build the project
cargo build --release

# Run the server
./target/release/tcp_chat
```

### Usage

**Start the server:**
```bash
cargo run --release
# Server listening on 127.0.0.1:8080
```

**Connect with netcat:**
```bash
nc 127.0.0.1 8080
```

**Or use a custom client:**
```bash
telnet 127.0.0.1 8080
```

## ⚙️ Configuration

The server supports configuration via a `config.json` file:

```json
{
  "host": "127.0.0.1",
  "port": 8080,
  "max_connections": 10000,
  "max_message_size": 4096,
  "idle_timeout_secs": 300,
  "connection_timeout_secs": 10,
  "rate_limit_msgs": 100,
  "rate_limit_window_secs": 1,
  "max_username_length": 32,
  "max_command_length": 512,
  "backlog": 1024,
  "message_buffer_size": 65536
}
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `host` | `String` | `127.0.0.1` | Server bind address |
| `port` | `u16` | `8080` | Listening port |
| `max_connections` | `usize` | `10,000` | Maximum concurrent clients |
| `max_message_size` | `usize` | `4096` | Max message size in bytes |
| `idle_timeout_secs` | `u64` | `300` | Idle connection timeout |
| `connection_timeout_secs` | `u64` | `10` | Connection accept timeout |
| `rate_limit_msgs` | `usize` | `100` | Rate limit per window |
| `rate_limit_window_secs` | `u64` | `1` | Rate limit window in seconds |
| `max_username_length` | `usize` | `32` | Max username length |
| `max_command_length` | `usize` | `512` | Max command length |
| `backlog` | `usize` | `1024` | TCP listener backlog |
| `message_buffer_size` | `usize` | `65536` | Message buffer size |

## 🎮 Client Commands

| Command | Description | Example |
|---------|-------------|---------|
| `/name <username>` | Change your display name | `/name Alice` |
| `/users` | List all online users | `/users` |
| `/quit` | Disconnect from server | `/quit` |

**Regular messages** are broadcast to all connected clients:
```
Hello everyone!
```

## 🛠️ Development

### Running Tests

```bash
cargo test
```

### Checking Code

```bash
cargo clippy -- -D warnings
cargo fmt --check
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## 📊 Performance

- **Async Runtime**: Tokio 1.x with multi-threaded scheduler
- **Memory Efficient**: Minimal per-connection overhead
- **Scalable**: Designed for 10K+ concurrent connections
- **Zero-Copy**: Where possible, avoids unnecessary allocations

## 🔧 Cargo.toml

```toml
[package]
name = "tcp_chat"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full", "rt-multi-thread", "macros", "net", "sync", "time", "io-util", "io-std"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json", "fmt", "ansi"] }
serde = { version = "1.0", features = ["derive"] }
chrono = "0.4"
thiserror = "1.0"
anyhow = "1.0"
dashmap = "5.5"
bytes = "1.5"

[dev-dependencies]
tokio-test = "0.4"
criterion = "0.5"

[profile.release]
opt-level = 3
lto = true
strip = true
codegen-units = 1
panic = "abort"

[profile.dev]
debug = true
```

### Security Benefits of Dependencies

| Dependency | Security Benefit |
|------------|------------------|
| **tokio** | Async runtime with cooperative multitasking, prevents blocking |
| **tracing** | Structured logging for security audit trails |
| **dashmap** | Lock-free concurrent data structures, avoids deadlock |
| **anyhow** | Consistent error handling, prevents unwrap() crashes |
| **bytes** | Zero-copy buffer management, prevents memory bugs |
| **chrono** | Safe timestamp handling for logs |

## 🔒 Security Engineering

### DoS Mitigation

```rust
// Rate limiting per client
if !self.rate_limiter.check_rate(self.client_id).await {
    return Err(anyhow!("Rate limit exceeded"));
}

// Connection limits via config
let config = Config {
    max_connections: 10_000,
    rate_limit_msgs: 100,
    ...
};
```

### Slowloris Defense

```rust
// Idle timeout disconnects inactive clients
let mut idle_timer = timeout_manager.create_idle_timer();

tokio::select! {
    result = reader.next_line() => { ... }
    _ = &mut idle_timer => {
        tracing::info!("Client {} idle timeout", self.client_id);
        break;
    }
}
```

### Bounded Queues

```rust
// Broadcast channel with fixed capacity
let (sender, _) = broadcast::channel(64);

// When full, send fails immediately
sender.send(msg)?;  // Returns error if channel full
```

### Timeout Strategy

```rust
// Connection timeout during accept
let result = timeout(Duration::from_secs(10), listener.accept()).await?;

// Idle timeout per client
let idle_timer = timeout(Duration::from_secs(config.idle_timeout_secs), ...);
```

### Panic Containment

```rust
tokio::spawn(async move {
    if let Err(e) = handle_client(...) {
        // Panic is contained within this task
        tracing::error!("Client handler error: {}", e);
    }
});
```

## ⚠️ Security Mistakes to Avoid

### 1. Trusting Client Input

```rust
// WRONG
let username = line.trim();

// RIGHT
let username = sanitizer.sanitize_username(line.trim())?;
```

### 2. Unbounded Memory Usage

```rust
// WRONG
let mut messages = Vec::new();

// RIGHT
let messages: VecDeque<Message> = VecDeque::with_capacity(100);
```

### 3. Holding Mutex Across Await

```rust
// WRONG
let mut guard = mutex.lock().await;
let result = some_async_op().await;

// RIGHT
{
    let mut guard = mutex.lock().await;
    guard.prepare();
}
let result = some_async_op().await;
```

### 4. Logging Sensitive Data

```rust
// WRONG
tracing::info!("Password: {}", password);

// RIGHT
tracing::info!("Client authenticated");
```

## 🚀 Production Hardening

- **TLS Integration**: Use `tokio-rustls` for encrypted connections
- **JWT Auth**: Implement token-based authentication
- **Database Persistence**: Use PostgreSQL via `sqlx`
- **Redis Pub/Sub**: For distributed chat across servers
- **WebSocket Gateway**: Use `tokio-tungstenite` for browser clients
- **Observability**: Integrate metrics and structured logging



## 🙏 Acknowledgments

- [Tokio](https://tokio.rs/) - Async runtime
- [Tracing](https://tracing.rs/) - Structured logging
- [Serde](https://serde.rs/) - Serialization framework

---

<p align="center">
  Made with ❤️ in Rust
</p>