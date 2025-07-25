# Rust TCP 3-Way Handshake Implementation

A comprehensive demonstration of TCP network programming in Rust, implementing a custom 3-way handshake protocol with multiple server architectures.

## 🤝 What is the 3-Way Handshake Protocol?

The 3-way handshake is a fundamental networking concept used to establish reliable connections. In this implementation, we've created a simplified version for educational purposes:

1. **Client → Server**: `HELLO X` (where X is initial sequence number)
2. **Server → Client**: `HELLO Y` (where Y = X + 1)
3. **Client → Server**: `HELLO Z` (where Z = Y + 1)

This exchange ensures both parties can send and receive messages correctly before proceeding with data transmission.

## 🚀 Applications Overview

This repository contains **4 different implementations** demonstrating various approaches to network programming in Rust:

### 🔹 Client (`client-sync.rs`)
A synchronous TCP client that initiates the 3-way handshake with any of the server implementations.

**Usage:**
```bash
cargo run --bin client-sync -- <server_ip> <server_port> <initial_sequence>
```

### 🔹 Sequential Server (`server-sequential.rs`)
A basic server that handles **one client at a time**. Simple but limited in scalability.

**Usage:**
```bash
cargo run --bin server-sequential -- <port>
```

### 🔹 Thread-per-Connection Server (`server-threaded.rs`)
Creates a **new thread for each client connection**, allowing concurrent client handling. Better scalability but with thread creation overhead.

**Usage:**
```bash
cargo run --bin server-threaded -- <port>
```

### 🔹 Thread Pool Server (`server-threadpool.rs`)
Uses a **fixed-size thread pool** to handle client connections efficiently. Optimal resource management with configurable worker threads.

**Usage:**
```bash
cargo run --bin server-threadpool -- <port>
```

## 🛠️ Building and Running

### Prerequisites
- Rust 1.70+ (uses 2021 edition)
- Cargo package manager

### Build Commands
```bash
# Build all applications
cargo build

# Build release versions (optimized)
cargo build --release

# Build specific application
cargo build --bin client-sync
cargo build --bin server-threadpool
```

### Example Usage
**Terminal 1 (Server):**
```bash
cargo run --bin server-threadpool -- 8080
```

**Terminal 2 (Client):**
```bash
cargo run --bin client-sync -- 127.0.0.1 8080 100
```

**Expected Output:**
- Server displays: `HELLO 100`, `HELLO 102`
- Client displays: `HELLO 101`

## 📚 Learning Resources

This project is part of a comprehensive blog series on Rust network programming:

- **[Episode 1: Introduction + Client + Sequential Server](https://debugndiscover.netlify.app/posts/rust-handshake-ep1/)** -- Basics of TCP programming in Rust and implementing the foundation
- **[Episode 2: Threaded + ThreadPool Servers](https://debugndiscover.netlify.app/posts/rust-handshake-ep2/)** -- Exploring concurrent server architectures
- **Episode 3: Event-driven (Async/Await)** *(Coming Soon)* -- Modern asynchronous programming approaches

## 🏗️ Architecture Highlights

- **Shared Library**: Common protocol logic and utilities to minimize code duplication
- **Error Handling**: Robust error management using `thiserror` for structured error types
- **Modular Design**: Clean separation between client/server logic and protocol implementation
- **Performance Optimized**: Automatic thread pool sizing based on system capabilities
- **Production Ready**: Proper timeout handling, connection management, and logging

## 📦 Dependencies

- [`threadpool`](https://crates.io/crates/threadpool) - Thread pool implementation for concurrent servers
- [`thiserror`](https://crates.io/crates/thiserror) - Structured error handling
- [`anyhow`](https://crates.io/crates/anyhow) - Flexible error handling utilities

## 🎯 Key Learning Objectives

- **TCP Socket Programming**: Understanding low-level network communication
- **Concurrency Patterns**: Comparing different approaches to handling multiple clients
- **Resource Management**: Thread pools vs thread-per-connection trade-offs
- **Error Handling**: Robust error propagation and user-friendly messages
- **Code Organization**: Building maintainable Rust projects with shared libraries

## 🔧 Testing

Test the different server implementations with multiple concurrent clients to observe performance characteristics:

```bash
# Terminal 1: Start server
cargo run --bin server-threadpool -- 8080

# Terminals 2-5: Launch multiple clients simultaneously
cargo run --bin client-sync -- 127.0.0.1 8080 1 &
cargo run --bin client-sync -- 127.0.0.1 8080 2 &
cargo run --bin client-sync -- 127.0.0.1 8080 3 &
cargo run --bin client-sync -- 127.0.0.1 8080 4 &
```

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 👨‍💻 Author

**Sae-Hwan Park**
- GitHub: [@SaehwanPark](https://github.com/SaehwanPark)
- Blog: [Debug & Discover](https://debugndiscover.netlify.app/)

---

⭐ **Star this repository** if you found it helpful for learning Rust network programming!
