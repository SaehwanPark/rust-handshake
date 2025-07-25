/**
 * Shared error types for 3-way Handshake Protocol
 *
 * Author: Sae-Hwan Park
 */
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandshakeError {
  #[error("IO error: {0}")]
  Io(#[from] io::Error),

  #[error("Invalid message format: expected 'HELLO <number>', got '{message}'")]
  InvalidMessageFormat { message: String },

  #[error("Invalid sequence number: {0}")]
  InvalidSequenceNumber(String),

  #[error("Sequence mismatch: expected {expected}, received {received}")]
  SequenceMismatch { expected: i32, received: i32 },

  #[error("Client disconnected unexpectedly")]
  ClientDisconnected,

  #[error("Connection timeout")]
  Timeout,

  #[error("Invalid port number: {0}")]
  InvalidPort(String),

  #[error("Invalid command line arguments: {0}")]
  InvalidArguments(String),
}

pub type Result<T> = std::result::Result<T, HandshakeError>;
