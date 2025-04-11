# Async Chat Client

[![Built with Tokio](https://img.shields.io/badge/built%20with-Tokio-%23ECD53F)](https://tokio.rs)
[![Rust](https://img.shields.io/badge/rust-1.72%2B-orange)](https://www.rust-lang.org)

An asynchronous command-line client for the async-chat-server, built with Rust and Tokio.

## Features
- 🚀 Non-blocking asynchronous I/O
- 📡 Simultaneous message reading/writing
- 💻 Simple terminal interface
- 🔄 Background message processing
- 🛑 Clean disconnect handling
- ⚙️ Basic error handling

## Planned Improvements
- Message formatting/timestamps
- User nickname support
- Tab completion
- Chat history scrolling
- Colored output
- Better connection recovery
- Interactive help commands

## Installation

### Prerequisites
- Rust 1.72+ ([rustup](https://rustup.rs/))
- Cargo package manager
- Running instance of [flycatcher-chat-server](https://github.com/izarma/flycatcher-chat-server)

```bash
git clone https://github.com/yourusername/async-chat-client.git
cd async-chat-client
cargo build --release
```

## Usage
```bash
# Connect to default server
cargo run -- --address localhost:8080

# Connect to custom server
cargo run -- --address your.server:port
```

Example session:
```
Connected to server at 127.0.0.1:8080
> Hello chat!
< Server> User joined (127.0.0.1:54321)
< Bob> Hey everyone!
> How's it going?
< Bob> Great! Just testing the chat
```

**Controls:**
- Type messages and press Enter to send
- `Ctrl+D` or `Ctrl+C` to exit
- Maximum message length: 1024 characters

## Implementation Details
The client uses Tokio's async primitives for efficient networking:
- `TcpStream` for server connection
- Split I/O using `tokio::io::split`
- Separate async tasks for:
  - Reading server messages
  - Handling user input
- Buffered I/O with `BufReader`

Key components:
1. Establishes TCP connection to server
2. Splits stream into separate read/write halves
3. Spawns independent tasks for:
   - Reading server messages and printing to stdout
   - Reading user input from stdin and sending to server
4. Uses async/await for non-blocking operations

## Contributing
Contributions are welcome! Please:
1. Open an issue to discuss proposed changes
2. Follow Rust coding conventions
3. Add tests for new features
4. Update documentation accordingly

## License
[MIT License](LICENSE)

---

Built with ❤️ using [Rust](https://www.rust-lang.org/) + [Tokio](https://tokio.rs)
