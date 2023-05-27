use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use simplelog::info;

use crate::network::tcp_listener::start_listening;

mod executors;
mod network;
mod utils;

fn main() {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:8070").expect("Failed to bind TCP listener");

    info!("Listening for connections on port 8070 ...");

    let listener_active = Arc::new(AtomicBool::new(false));
    let shutdown_requested = Arc::new(AtomicBool::new(false));
    let child_process = Arc::new(Mutex::new(None));

    start_listening(
        listener,
        listener_active.clone(),
        shutdown_requested.clone(),
        child_process.clone(),
    );

    wait_for_shutdown(listener_active);
}

fn wait_for_shutdown(listener_active: Arc<AtomicBool>) {
    while listener_active.load(Ordering::SeqCst) {
        thread::yield_now();
    }
}
