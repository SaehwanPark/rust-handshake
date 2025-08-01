/**
 * Event-Driven Server for 3-way Handshake Protocol
 *
 * Author: Sae-Hwan Park
 *
 * This server uses async/await with Tokio runtime to efficiently handle
 * multiple client connections concurrently without creating explicit threads.
 * Each connection is handled as a lightweight async task.
 */
use std::net::SocketAddr;
use tokio::net::TcpStream;

use tcp_handshake::{
  create_async_listener, exit_with_error, parse_server_args, perform_async_server_handshake,
};

/**
 * Async task wrapper to handle client connections
 * Ensures proper error handling and logging
 */
async fn handle_client_task(stream: TcpStream, peer_addr: SocketAddr) {
  match perform_async_server_handshake(stream, peer_addr).await {
    Ok(_) => {
      println!("Successfully handled connection from {peer_addr}");
    }
    Err(e) => {
      eprintln!("ERROR handling {peer_addr}: {e}");
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Parse command line arguments
  let port = match parse_server_args() {
    Ok(port) => port,
    Err(e) => exit_with_error(&e),
  };

  // Create and bind async listener
  let listener = match create_async_listener(port).await {
    Ok(listener) => listener,
    Err(e) => exit_with_error(&e),
  };

  // Main async event loop
  // Accept connections and spawn async tasks to handle them
  loop {
    match listener.accept().await {
      Ok((stream, peer_addr)) => {
        println!("Accepted connection from {peer_addr}");

        // Spawn a new async task to handle this client concurrently
        // The task will run independently and not block other connections
        tokio::spawn(async move {
          handle_client_task(stream, peer_addr).await;
        });
      }
      Err(e) => {
        eprintln!("ERROR accepting connection: {e}");
        // Continue accepting other connections
        continue;
      }
    }
  }
}
