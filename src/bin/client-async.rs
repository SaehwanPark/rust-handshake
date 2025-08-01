use tcp_handshake::{
  exit_with_error, format_server_address, parse_client_args, perform_async_client_handshake,
};
use tokio::net::TcpStream;

/**
 * Event-Driven Client for 3-way Handshake Protocol
 *
 * Author: Sae-Hwan Park
 *
 * This async client implementation uses Tokio runtime for
 * non-blocking I/O operations with the event-driven server
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Parse command line arguments
  let (server_ip, port, initial_seq) = match parse_client_args() {
    Ok(args) => args,
    Err(e) => exit_with_error(&e),
  };

  // Connect to the server asynchronously
  let server_addr = format_server_address(&server_ip, port);
  println!("Connecting to {server_addr}...");

  let stream = match TcpStream::connect(&server_addr).await {
    Ok(stream) => {
      println!("Connected to {server_addr}");
      stream
    }
    Err(e) => {
      eprintln!("ERROR: Failed to connect to {server_addr}: {e}");
      std::process::exit(1);
    }
  };

  // Perform the 3-way handshake asynchronously
  if let Err(e) = perform_async_client_handshake(stream, initial_seq).await {
    exit_with_error(&e);
  }

  println!("Client completed successfully!");
  Ok(())
}
