use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::ChildStdin;
use std::thread;
use std::time::Duration;

use simplelog::{debug, error, info, warn};

pub fn start_message_injector(
    mut stream: TcpStream,
    mut child_stdin: ChildStdin,
) {
    thread::spawn(move || {
        let timeout_duration = Duration::from_secs(10);

        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(bytes_read) => {
                    info!(
                        "Message received from client: {}",
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    );

                    match child_stdin.write_all(&buffer[..bytes_read]) {
                        Ok(_) => debug!("Message sent to Python script"),
                        Err(e) => error!("Failed to write to Python script: {}", e)
                    }

                    match stream.set_read_timeout(Some(timeout_duration)) {
                        Ok(_) => debug!("Set read timeout for TCP stream"),
                        Err(e) => error!("Failed to set read timeout for TCP stream: {}", e)
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
                    warn!("Connection reset by client");
                    break;
                }
                Err(e) => {
                    error!("Failed to read from TCP stream: {}", e);
                    break;
                }
            }
        }
    });
}
