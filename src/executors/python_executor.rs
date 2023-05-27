use std::process::{Child, Command, Stdio};

pub fn python_executor() -> Child {
    Command::new("python")
        .args(&["game.py"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start Python script")
}
