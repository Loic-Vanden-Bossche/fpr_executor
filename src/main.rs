use crate::core::start::start_fpr_executor;

mod executors;
mod network;
mod utils;
mod core;

fn main() {
    start_fpr_executor();
}