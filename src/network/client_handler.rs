use std::net::TcpStream;
use std::process::Child;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use simplelog::info;

use crate::executors::executor::game_executor;
use crate::executors::types::ExecutorType;
use crate::network::message_extractor::start_message_extractor;
use crate::network::message_injector::start_message_injector;

pub fn handle_client(
    stream: TcpStream,
    listener_active: Arc<AtomicBool>,
    shutdown_requested: Arc<AtomicBool>,
    child_process: Arc<Mutex<Option<Child>>>,
) {
    listener_active.store(true, Ordering::SeqCst);

    let cloned_stream = stream.try_clone().expect("Failed to clone TCP stream");

    info!("Starting Python script ...");
    let mut child = game_executor(ExecutorType::Python);

    info!("Python script started, listening for messages ...");

    let child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");

    let json_buffer = String::new();

    start_message_extractor(
        child_stdout,
        cloned_stream
            .try_clone()
            .expect("Failed to clone TCP stream"),
        json_buffer.clone(),
    );

    start_message_injector(stream, child_stdin, shutdown_requested);

    info!("Closing game executor ...");

    let mut child_process_guard = child_process.lock().unwrap();
    if let Some(mut child) = child_process_guard.take() {
        let _ = child.kill();
        let _ = child.wait();
    }

    listener_active.store(false, Ordering::SeqCst);
}
