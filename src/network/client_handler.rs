use std::net::TcpStream;

use simplelog::{debug, error, info};

use crate::executors::executor::game_executor;
use crate::executors::types::ExecutorType;
use crate::network::message_extractor::start_message_extractor;
use crate::network::message_injector::start_message_injector;

pub fn handle_client(
    stream: TcpStream,
    executor_type: ExecutorType,
    script_path: String,
) {
    match stream.set_nonblocking(true) {
        Ok(_) => debug!("Set non-blocking mode for cloned client stream"),
        Err(err) => {
            error!("Failed to set non-blocking mode for cloned client stream: {}", err);
            return;
        }
    };

    let cloned_stream = match stream.try_clone() {
        Ok(stream) => stream,
        Err(_) => {
            error!("Failed to clone TCP stream");
            return;
        }
    };

    info!("Starting Python script ...");
    let mut child = game_executor(executor_type, script_path);

    info!("Python script started, listening for messages ...");

    let child_stdin = match child.stdin.take() {
        Some(stdin) => stdin,
        None => {
            error!("Failed to open stdin in game process");
            return;
        }
    };
    let child_stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            error!("Failed to open stdout in game process");
            return;
        }
    };

    start_message_extractor(child_stdout, cloned_stream);

    start_message_injector(stream, child_stdin);

    match child.wait() {
        Ok(_) => info!("Game process exited successfully"),
        Err(status) => error!("Game process exited with error: {}", status.to_string()),
    };

    info!("Closing game process ...");

    match child.kill() {
        Ok(_) => info!("Game process closed successfully"),
        Err(_) => info!("Game process already closed"),
    };
}
