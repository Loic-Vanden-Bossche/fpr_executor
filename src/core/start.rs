use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use simplelog::{info};
use crate::core::args::parse_args;
use crate::core::logs::init_logs;
use crate::network::tcp_listener::start_listening;

pub fn start_fpr_executor() {
    let (executor_type, port, log_level) = parse_args();

    init_logs(log_level);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind TCP listener");

    info!("Starting executor with type {:?} on port {}", executor_type, port);

    let listener_active = Arc::new(AtomicBool::new(false));
    let shutdown_requested = Arc::new(AtomicBool::new(false));
    let child_process = Arc::new(Mutex::new(None));

    start_listening(
        listener,
        listener_active.clone(),
        shutdown_requested.clone(),
        child_process.clone(),
        executor_type,
    );

    wait_for_shutdown(listener_active);
}

fn wait_for_shutdown(listener_active: Arc<AtomicBool>) {
    while listener_active.load(Ordering::SeqCst) {
        thread::yield_now();
    }
}
