use simplelog::{ColorChoice, Config, debug, LevelFilter, TerminalMode};

pub fn init_logs(log_level: LevelFilter) {
    match simplelog::TermLogger::init(log_level, Config::default(), TerminalMode::Mixed, ColorChoice::Always) {
        Ok(_) => { debug!("Logger loaded") }
        Err(err) => {
            println!("Error on loading logger: {err}")
        }
    }
}