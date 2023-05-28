use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use simplelog::{debug, error, info, warn};

use crate::executors::types::ExecutorType;
use crate::network::client_handler::handle_client;

pub fn start_listening(listener: TcpListener, executor_type: ExecutorType) {
    let listener_active = Arc::new(AtomicBool::new(false));
    let should_quit = Arc::new(AtomicBool::new(false));

    match listener.set_nonblocking(true) {
        Ok(_) => debug!("Set non-blocking mode for listener"),
        Err(err) => {
            error!("Failed to set non-blocking mode for listener: {}", err);
            return;
        }
    };

    loop {
        if should_quit.load(Ordering::SeqCst) {
            break;
        }

        if let Ok((stream, _)) = listener.accept() {
            debug!("New client connection incoming ...");

            if listener_active.load(Ordering::SeqCst) {
                warn!("Another listener is already active. Skipping incoming connection.");
                continue;
            }

            let peer_addr = match stream.peer_addr() {
                Ok(peer_addr) => peer_addr.to_string(),
                Err(err) => {
                    error!("Failed to get peer address: {}", err);
                    "Unknown".to_string()
                }
            };

            info!("New client connection accepted from: {}", peer_addr);
            debug!("Setting listener active flag to true ...");

            listener_active.store(true, Ordering::SeqCst);

            let cloned_listener_active = listener_active.clone();
            let cloned_should_quit = should_quit.clone();

            thread::spawn(move || {
                handle_client(stream, executor_type);

                info!("Closing game executor ...");

                cloned_listener_active.store(false, Ordering::SeqCst);
                cloned_should_quit.store(true, Ordering::SeqCst);
            });
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
}