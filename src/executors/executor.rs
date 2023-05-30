use std::process::Child;

use crate::executors::python_executor::python_executor;
use crate::executors::types::ExecutorType;

pub fn game_executor(executor_type: ExecutorType, script_path: String) -> Child {
    match executor_type {
        ExecutorType::Python => python_executor(script_path),
        ExecutorType::Node => {
            unimplemented!("Node not implemented yet");
        }
        ExecutorType::Binary => {
            unimplemented!("Binary not implemented yet");
        }
    }
}
