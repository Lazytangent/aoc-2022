use clap::Parser;

use crate::fs::DataType;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Data file type to read
    #[arg(short = 't', long = "type", default_value_t = DataType::Sample)]
    pub r#type: DataType,
}

pub fn run() -> Cli {
    Cli::parse()
}
