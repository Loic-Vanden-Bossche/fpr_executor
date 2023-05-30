use std::process::{Child, Command, Stdio};

pub fn python_executor(script_path: String) -> Child {
    match Command::new("python")
        .args(&[script_path])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Ok(child) => child,
        Err(e) => panic!("Failed to start Python script: {}", e),
    }
}
