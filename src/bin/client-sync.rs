/**
 * Basic Client for 3-way Handshake Protocol
 *
 * Author: Sae-Hwan Park
 */
use std::net::TcpStream;
use tcp_handshake::{
  exit_with_error, format_server_address, parse_client_args, perform_client_handshake,
};

fn main() {
  // Parse command line arguments
  let (server_ip, port, initial_seq) = match parse_client_args() {
    Ok(args) => args,
    Err(e) => exit_with_error(&e),
  };

  // Connect to the server
  let server_addr = format_server_address(&server_ip, port);
  let stream = match TcpStream::connect(&server_addr) {
    Ok(stream) => stream,
    Err(e) => {
      eprintln!("ERROR: Failed to connect to {server_addr}: {e}");
      std::process::exit(1);
    }
  };

  // Perform the 3-way handshake
  if let Err(e) = perform_client_handshake(stream, initial_seq) {
    exit_with_error(&e);
  }

  // Handshake completed successfully
}
