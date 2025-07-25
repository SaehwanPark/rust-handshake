/**
 * Shared library for 3-way Handshake Protocol
 *
 * Author: Sae-Hwan Park
 */
pub mod error;
pub mod protocol;
pub mod utils;

// Re-export commonly used items
pub use error::{HandshakeError, Result};
pub use protocol::{
  format_hello_message, parse_hello_message, perform_client_handshake, perform_server_handshake,
  read_message_from_stream, write_message_to_stream,
};
pub use utils::{
  calculate_optimal_thread_count, create_listener, exit_with_error, format_server_address,
  parse_client_args, parse_server_args,
};

pub const MSG_SIZE: usize = 64;
