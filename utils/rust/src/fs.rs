use std::{fmt, fs, io::Result, process};

use clap::ValueEnum;

use crate::constants::{REAL_DATA, SAMPLE_DATA};

fn read_file(filename: &str) -> Result<String> {
    fs::read_to_string(filename)
}

pub fn read_sample_data() -> Result<String> {
    read_file(SAMPLE_DATA)
}

pub fn read_real_data() -> Result<String> {
    read_file(REAL_DATA)
}

pub fn read_data(r#type: DataType) -> String {
    let func = match r#type {
        DataType::Real => read_real_data,
        DataType::Sample => read_sample_data,
    };

    match func() {
        Ok(f) => f.trim_end().to_string(),
        Err(e) => {
            eprintln!("Error: {e:#?}");
            process::exit(1);
        }
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    Real,
    Sample,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DataType::Real => write!(f, "real"),
            DataType::Sample => write!(f, "sample"),
        }
    }
}
