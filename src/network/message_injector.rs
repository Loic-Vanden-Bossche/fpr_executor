use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::ChildStdin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use simplelog::{error, info, warn};

pub fn start_message_injector(
    mut stream: TcpStream,
    mut child_stdin: ChildStdin,
    shutdown_requested: Arc<AtomicBool>,
) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                info!(
                    "Message received from client: {}",
                    String::from_utf8_lossy(&buffer[..bytes_read])
                );

                child_stdin
                    .write_all(&buffer[..bytes_read])
                    .expect("Failed to write to stdin");
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::ConnectionReset {
                    warn!("Connection reset by client");
                    shutdown_requested.store(true, Ordering::SeqCst);
                    break;
                }

                error!("Failed to read from TCP stream: {}", e);
            }
        }
    }
}
