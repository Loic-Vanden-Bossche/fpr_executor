use std::net::TcpListener;

use simplelog::{error, info};

use crate::core::args::parse_args;
use crate::core::logs::init_logs;
use crate::network::tcp_listener::start_listening;
use crate::utils::path_checker::check_game_script_path;

pub fn start_fpr_executor() {
    let (executor_type, port, log_level, script_path, listener_timeout) = parse_args();

    init_logs(log_level);

    match check_game_script_path(&script_path) {
        Ok(full_path) => {
            info!("Game script path is valid: {}", script_path);
            full_path
        }
        Err(err) => {
            error!("Failed to check game script path: {}", err);
            std::process::exit(1);
        }
    }

    let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(listener) => listener,
        Err(err) => {
            info!("Failed to bind to port {}: {}", port, err);
            return;
        }
    };

    info!("Starting executor with type {:?} on port {}", executor_type, port);

    start_listening(listener, executor_type, script_path, listener_timeout);
}
