/**
 * Shared protocol-related helpers for 3-way Handshake
 *
 * Author: Sae-Hwan Park
 */
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

// Async imports
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::time::timeout;

use crate::MSG_SIZE;
use crate::error::{HandshakeError, Result};

// Timeout constants for async operations
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
pub const READ_TIMEOUT: Duration = Duration::from_secs(5);
pub const CLIENT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

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
 * Async version: Reads a message from TCP stream with timeout
 */
pub async fn read_message_from_async_stream(stream: &mut AsyncTcpStream) -> Result<String> {
  let mut buffer = [0u8; MSG_SIZE];

  let bytes_read = timeout(READ_TIMEOUT, stream.read(&mut buffer))
    .await
    .map_err(|_| HandshakeError::Timeout)?
    .map_err(HandshakeError::Io)?;

  if bytes_read == 0 {
    return Err(HandshakeError::ClientDisconnected);
  }

  let message = String::from_utf8_lossy(&buffer[..bytes_read]);
  let message = message.trim_end_matches('\0').trim();

  Ok(message.to_string())
}

/**
 * Async version: Writes a message to TCP stream
 */
pub async fn write_message_to_async_stream(
  stream: &mut AsyncTcpStream,
  message: &str,
) -> Result<()> {
  stream.write_all(message.as_bytes()).await?;
  Ok(())
}

/**
 * Async version: Performs client-side 3-way handshake
 */
pub async fn perform_async_client_handshake(
  mut stream: AsyncTcpStream,
  initial_seq: i32,
) -> Result<()> {
  // Wrap entire handshake in timeout
  let result = timeout(CLIENT_CONNECTION_TIMEOUT, async {
    // Step 1: Send HELLO X where X is initial sequence
    let first_message = format_hello_message(initial_seq);
    write_message_to_async_stream(&mut stream, &first_message).await?;
    println!("Sent: {first_message}");

    // Step 2: Receive HELLO Y and validate Y = X + 1
    let received_msg = read_message_from_async_stream(&mut stream).await?;

    // Print received message to stdout
    println!("Received: {received_msg}");
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
    write_message_to_async_stream(&mut stream, &final_message).await?;
    println!("Sent: {final_message}");

    println!("Handshake completed successfully!");
    Ok::<(), HandshakeError>(())
  })
  .await
  .map_err(|_| HandshakeError::Timeout)?;

  result
}

/**
 * Async version: Performs server-side 3-way handshake
 */
pub async fn perform_async_server_handshake(
  mut stream: AsyncTcpStream,
  peer_addr: std::net::SocketAddr,
) -> Result<()> {
  println!("Handling connection from {peer_addr}");

  // Wrap the entire handshake in a timeout to prevent hanging connections
  let result = timeout(CONNECTION_TIMEOUT, async {
    // Step 1: Receive HELLO X
    let received_msg = read_message_from_async_stream(&mut stream).await?;

    // Print received message
    println!("Received from {peer_addr}: {received_msg}");
    std::io::Write::flush(&mut std::io::stdout())?;

    // Parse the client's sequence number
    let client_seq = parse_hello_message(&received_msg)?;

    // Step 2: Send HELLO Y where Y = X + 1
    let server_seq = client_seq + 1;
    let response = format_hello_message(server_seq);
    write_message_to_async_stream(&mut stream, &response).await?;
    println!("Sent to {peer_addr}: {response}");

    // Step 3: Receive HELLO Z and validate Z = Y + 1
    let final_msg = read_message_from_async_stream(&mut stream).await?;

    // Print received message
    println!("Received from {peer_addr}: {final_msg}");
    std::io::Write::flush(&mut std::io::stdout())?;

    // Parse and validate final sequence number
    let final_seq = parse_hello_message(&final_msg)?;
    let expected_final = server_seq + 1;

    if final_seq != expected_final {
      eprintln!(
        "ERROR: Expected HELLO {expected_final}, received HELLO {final_seq} from {peer_addr}"
      );
    }

    println!("Handshake completed successfully with {peer_addr}");
    Ok::<(), HandshakeError>(())
  })
  .await
  .map_err(|_| HandshakeError::Timeout)?;

  result
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
