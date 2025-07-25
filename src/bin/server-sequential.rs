/**
 * Sequential Server for 3-way Handshake Protocol
 * Handles one client at a time
 *
 * Author: Sae-Hwan Park
 */
use tcp_handshake::{
  create_listener, exit_with_error, parse_server_args, perform_server_handshake,
};

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

  // Main server loop - handle one client at a time
  loop {
    match listener.accept() {
      Ok((stream, addr)) => {
        println!("Accepted connection from {addr}");
        if let Err(e) = perform_server_handshake(stream) {
          eprintln!("ERROR: Handshake failed with {addr}: {e}");
        }
        // Continue to next client regardless of handshake result
      }
      Err(e) => {
        eprintln!("ERROR: Failed to accept connection: {e}");
        // Continue listening for new connections
      }
    }
  }
}
