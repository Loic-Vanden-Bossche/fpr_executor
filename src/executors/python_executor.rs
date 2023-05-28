use std::process::{Child, Command, Stdio};

pub fn python_executor() -> Child {
    match Command::new("python")
        .args(&["game.py"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Ok(child) => child,
        Err(e) => panic!("Failed to start Python script: {}", e),
    }
}
