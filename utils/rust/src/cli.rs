use clap::Parser;

use crate::fs::DataType;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Data file type to read
    #[arg(short = 't', long = "type", default_value_t = DataType::Sample)]
    pub r#type: DataType,
    /// Part to run (1 or 2)
    #[arg(short, long, default_value_t = 1)]
    pub part: u8,
}

pub fn run() -> Cli {
    Cli::parse()
}
