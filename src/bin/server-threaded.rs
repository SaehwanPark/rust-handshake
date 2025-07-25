/**
 * Multi-threaded Server for 3-way Handshake Protocol
 * Creates a separate thread for each client connection
 *
 * Author: Sae-Hwan Park
 */
use std::net::TcpStream;
use std::thread;

use tcp_handshake::{
  create_listener, exit_with_error, parse_server_args, perform_server_handshake,
};

/**
 * Thread wrapper function to handle client connections
 */
fn handle_client_thread(stream: TcpStream) {
  let peer_addr = stream
    .peer_addr()
    .map(|addr| addr.to_string())
    .unwrap_or_else(|_| "unknown".to_string());

  match perform_server_handshake(stream) {
    Ok(_) => println!("Successfully handled connection from {peer_addr}"),
    Err(e) => eprintln!("ERROR: Handshake failed with {peer_addr}: {e}"),
  }
  // Thread automatically cleans up when function returns
}

fn main() {
  // Parse command line arguments
  let port = match parse_server_args() {
    Ok(port) => port,
    Err(e) => exit_with_error(&e),
  };

  // Create and bind listener
  let listener = match create_listener(port) {
    Ok(listener) => listener,
    Err(e) => exit_with_error(&e),
  };

  // Main server loop - spawn thread for each client
  loop {
    match listener.accept() {
      Ok((stream, addr)) => {
        println!("Accepted connection from {addr}");

        // Create a new thread to handle this client
        // Move the stream into the thread to transfer ownership
        thread::spawn(move || {
          handle_client_thread(stream);
        });
      }
      Err(e) => {
        eprintln!("ERROR: Failed to accept connection: {e}");
        // Continue accepting other connections
      }
    }
  }
}
