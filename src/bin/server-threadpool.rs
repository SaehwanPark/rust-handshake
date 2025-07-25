/**
 * Thread Pool Server for 3-way Handshake Protocol
 * Uses a thread pool to efficiently handle multiple client connections
 *
 * Author: Sae-Hwan Park
 */
use std::net::TcpStream;
use threadpool::ThreadPool;

use tcp_handshake::{
  calculate_optimal_thread_count, create_listener, exit_with_error, parse_server_args,
  perform_server_handshake,
};

/**
 * Worker function to handle client connection in thread pool
 * Ensures proper error handling and logging
 */
fn handle_client_worker(stream: TcpStream) {
  let peer_addr = stream
    .peer_addr()
    .map(|addr| addr.to_string())
    .unwrap_or_else(|_| "unknown".to_string());

  match perform_server_handshake(stream) {
    Ok(_) => println!("Successfully handled connection from {peer_addr}"),
    Err(e) => eprintln!("ERROR: Handshake failed with {peer_addr}: {e}"),
  }
}

fn main() {
  // Parse command line arguments
  let port = match parse_server_args() {
    Ok(port) => port,
    Err(e) => exit_with_error(&e),
  };

  // Determine optimal thread pool size
  // For I/O bound tasks like TCP handling, we can use more threads than CPU cores
  let num_threads = calculate_optimal_thread_count();
  println!("Starting server on port {port} with {num_threads} worker threads");

  // Create thread pool
  let pool = ThreadPool::new(num_threads);

  // Create and bind listener
  let listener = match create_listener(port) {
    Ok(listener) => listener,
    Err(e) => exit_with_error(&e),
  };

  // Main server loop - submit connections to thread pool
  loop {
    match listener.accept() {
      Ok((stream, addr)) => {
        println!("Accepted connection from {addr}");

        // Submit client handling to thread pool
        pool.execute(move || {
          handle_client_worker(stream);
        });
      }
      Err(e) => {
        eprintln!("ERROR: Failed to accept connection: {e}");
        // Continue accepting other connections
      }
    }
  }
}
