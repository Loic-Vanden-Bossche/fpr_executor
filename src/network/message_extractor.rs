use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::process::ChildStdout;
use std::thread;

use simplelog::{debug, info};

use crate::utils::json_validator::is_valid_json;

pub fn start_message_extractor(
    child_stdout: ChildStdout,
    mut cloned_stream: TcpStream,
    mut json_buffer: String,
) {
    thread::spawn(move || {
        let reader = std::io::BufReader::new(child_stdout);

        for line in reader.lines() {
            if let Ok(line) = line {
                debug!("Message received from Python script: {}", line);

                let is_valid_json = is_valid_json(&json_buffer, &line);

                json_buffer.push_str(&line.trim());

                if is_valid_json {
                    info!("Sending JSON to client: {}", json_buffer);

                    cloned_stream
                        .write_all(json_buffer.as_bytes())
                        .expect("Failed to write to TCP stream");
                    cloned_stream.flush().expect("Failed to flush TCP stream");

                    json_buffer.clear();
                }
            }
        }
    });
}
