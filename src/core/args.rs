use clap::Parser;
use simplelog::LevelFilter;

use crate::executors::types::ExecutorType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // The port to connect to
    #[arg(short, long, default_value_t = 8070)]
    pub port: u32,

    #[clap(short, long, value_enum, default_value = "binary")]
    pub exec_type: ExecutorType,

    #[arg(long)]
    pub debug: bool,
}

pub fn parse_args() -> (ExecutorType, u32, LevelFilter) {
    let args = Args::parse();
    (args.exec_type, args.port, if args.debug { LevelFilter::Debug } else { LevelFilter::Info })
}