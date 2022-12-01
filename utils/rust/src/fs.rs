use std::{fs, io::Result};

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
