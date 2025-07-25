/**
 * Shared protocol-related helpers for 3-way Handshake
 *
 * Author: Sae-Hwan Park
 */
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::MSG_SIZE;
use crate::error::{HandshakeError, Result};

/**
 * Parses a HELLO message and extracts the sequence number
 */
pub fn parse_hello_message(message: &str) -> Result<i32> {
  let parts: Vec<&str> = message.split_whitespace().collect();

  if parts.len() != 2 || parts[0] != "HELLO" {
    return Err(HandshakeError::InvalidMessageFormat {
      message: message.to_string(),
    });
  }

  parts[1]
    .parse::<i32>()
    .map_err(|_| HandshakeError::InvalidSequenceNumber(parts[1].to_string()))
}

/**
 * Formats a HELLO message with the given sequence number
 */
pub fn format_hello_message(seq_num: i32) -> String {
  format!("HELLO {seq_num}")
}

/**
 * Reads a message from TCP stream with timeout
 */
pub fn read_message_from_stream(stream: &mut TcpStream) -> Result<String> {
  let mut buffer = [0u8; MSG_SIZE];

  let bytes_read = stream.read(&mut buffer)?;
  if bytes_read == 0 {
    return Err(HandshakeError::ClientDisconnected);
  }

  let message = String::from_utf8_lossy(&buffer[..bytes_read]);
  let message = message.trim_end_matches('\0');

  Ok(message.to_string())
}

/**
 * Writes a message to TCP stream
 */
pub fn write_message_to_stream(stream: &mut TcpStream, message: &str) -> Result<()> {
  stream.write_all(message.as_bytes())?;
  Ok(())
}

/**
 * Performs client-side 3-way handshake
 */
pub fn perform_client_handshake(mut stream: TcpStream, initial_seq: i32) -> Result<()> {
  // Set read timeout for client
  stream.set_read_timeout(Some(Duration::from_secs(5)))?;

  // Step 1: Send HELLO X where X is initial sequence
  let first_message = format_hello_message(initial_seq);
  write_message_to_stream(&mut stream, &first_message)?;

  // Step 2: Receive HELLO Y and validate Y = X + 1
  let received_msg = read_message_from_stream(&mut stream)?;

  // Print received message to stdout
  println!("{received_msg}");
  std::io::Write::flush(&mut std::io::stdout())?;

  // Parse and validate
  let received_seq = parse_hello_message(&received_msg)?;
  let expected_seq = initial_seq + 1;

  if received_seq != expected_seq {
    return Err(HandshakeError::SequenceMismatch {
      expected: expected_seq,
      received: received_seq,
    });
  }

  // Step 3: Send HELLO Z where Z = Y + 1
  let final_seq = received_seq + 1;
  let final_message = format_hello_message(final_seq);
  write_message_to_stream(&mut stream, &final_message)?;

  Ok(())
}

/**
 * Performs server-side 3-way handshake
 */
pub fn perform_server_handshake(mut stream: TcpStream) -> Result<()> {
  // Set read timeout for server
  stream.set_read_timeout(Some(Duration::from_secs(5)))?;

  // Step 1: Receive HELLO X
  let received_msg = read_message_from_stream(&mut stream)?;

  // Print received message
  println!("{received_msg}");
  std::io::Write::flush(&mut std::io::stdout())?;

  // Parse the client's sequence number
  let client_seq = parse_hello_message(&received_msg)?;

  // Step 2: Send HELLO Y where Y = X + 1
  let server_seq = client_seq + 1;
  let response = format_hello_message(server_seq);
  write_message_to_stream(&mut stream, &response)?;

  // Step 3: Receive HELLO Z and validate Z = Y + 1
  let final_msg = read_message_from_stream(&mut stream)?;

  // Print received message
  println!("{final_msg}");
  std::io::Write::flush(&mut std::io::stdout())?;

  // Parse and validate final sequence number
  let final_seq = parse_hello_message(&final_msg)?;
  let expected_final = server_seq + 1;

  if final_seq != expected_final {
    eprintln!("ERROR: Expected HELLO {expected_final}, received HELLO {final_seq}");
  }

  Ok(())
}
