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
    /// Whether or not to use the real data
    #[arg(short, long, default_value_t = false)]
    pub real: bool,
}

pub fn run() -> Cli {
    Cli::parse()
}

pub fn get_type(cli: &Cli) -> DataType {
    match (cli.r#type, cli.real) {
        (DataType::Sample, false) => DataType::Sample,
        (DataType::Real, false) => DataType::Real,
        (DataType::Sample, true) => DataType::Real,
        (DataType::Real, true) => DataType::Real,
        (DataType::SampleTwo, _) => DataType::SampleTwo,
    }
}
