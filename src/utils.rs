/**
 * Shared helper functions for 3-way Handshake protocol
 *
 * Author: Sae-Hwan Park
 */
use std::env;
use std::net::TcpListener;
use std::process;

use crate::error::{HandshakeError, Result};

/**
 * Parses client command line arguments
 * Returns (server_ip, port, initial_sequence)
 */
pub fn parse_client_args() -> Result<(String, u16, i32)> {
  let args: Vec<String> = env::args().collect();

  if args.len() != 4 {
    return Err(HandshakeError::InvalidArguments(format!(
      "Usage: {} <server_ip> <server_port> <initial_sequence>",
      args[0]
    )));
  }

  let server_ip = args[1].clone();

  let port: u16 = args[2]
    .parse()
    .map_err(|_| HandshakeError::InvalidPort(args[2].clone()))?;

  let initial_seq: i32 = args[3]
    .parse()
    .map_err(|_| HandshakeError::InvalidSequenceNumber(args[3].clone()))?;

  Ok((server_ip, port, initial_seq))
}

/**
 * Parses server command line arguments
 * Returns port number
 */
pub fn parse_server_args() -> Result<u16> {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    return Err(HandshakeError::InvalidArguments(format!(
      "Usage: {} <server_port>",
      args[0]
    )));
  }

  let port: u16 = args[1]
    .parse()
    .map_err(|_| HandshakeError::InvalidPort(args[1].clone()))?;

  Ok(port)
}

/**
 * Creates and binds a TCP listener
 */
pub fn create_listener(port: u16) -> Result<TcpListener> {
  let bind_addr = format!("0.0.0.0:{port}");
  let listener = TcpListener::bind(&bind_addr).map_err(|e| HandshakeError::Io(e))?;

  println!("Listening on {bind_addr}");
  Ok(listener)
}

/**
 * Formats a socket address for display
 */
pub fn format_server_address(ip: &str, port: u16) -> String {
  format!("{ip}:{port}")
}

/**
 * Calculates optimal thread pool size for I/O bound tasks
 */
pub fn calculate_optimal_thread_count() -> usize {
  std::cmp::max(
    4,
    std::thread::available_parallelism()
      .map(|n| n.get() * 2)
      .unwrap_or(8),
  )
}

/**
 * Handles program exit with error message
 */
pub fn exit_with_error(error: &HandshakeError) -> ! {
  eprintln!("ERROR: {error}");
  process::exit(1);
}
