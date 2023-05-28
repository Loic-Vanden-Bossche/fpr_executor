use std::net::TcpListener;

use simplelog::info;

use crate::core::args::parse_args;
use crate::core::logs::init_logs;
use crate::network::tcp_listener::start_listening;

pub fn start_fpr_executor() {
    let (executor_type, port, log_level) = parse_args();

    init_logs(log_level);

    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(listener) => listener,
        Err(err) => {
            info!("Failed to bind to port {}: {}", port, err);
            return;
        }
    };

    info!("Starting executor with type {:?} on port {}", executor_type, port);

    start_listening(listener, executor_type);
}
