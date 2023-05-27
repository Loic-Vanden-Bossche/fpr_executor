use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use simplelog::{debug, error, info, warn};

use crate::network::client_handler::handle_client;

pub fn start_listening(
    listener: TcpListener,
    listener_active: Arc<AtomicBool>,
    shutdown_requested: Arc<AtomicBool>,
    child_process: Arc<Mutex<Option<std::process::Child>>>,
) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                debug!("New client connection incoming ...");

                if listener_active.load(Ordering::SeqCst) {
                    warn!("Another listener is already active. Skipping incoming connection.");
                    continue;
                }

                info!(
                    "New client connection accepted from: {}",
                    stream.peer_addr().unwrap()
                );
                debug!("Setting listener active flag to true ...");

                listener_active.store(true, Ordering::SeqCst);

                handle_client(
                    stream,
                    Arc::clone(&listener_active),
                    Arc::clone(&shutdown_requested),
                    Arc::clone(&child_process),
                );
            }
            Err(e) => {
                error!("Failed to accept client connection: {}", e);
            }
        }

        if shutdown_requested.load(Ordering::SeqCst) {
            info!("Shutdown requested. Closing listener.");
            break;
        }
    }
}
