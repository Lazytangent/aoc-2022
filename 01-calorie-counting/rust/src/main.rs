use std::{fs, io::Result, process};

const SAMPLE_DATA: &str = "data/small.txt";
const REAL_DATA: &str = "data/full.txt";

fn main() {
    let contents = match read_file(SAMPLE_DATA) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {e:#?}");
            process::exit(1);
        }
    };

    let elves: Vec<String> = contents
        .split("\n\n")
        .map(String::from)
        .collect();

    let mut highest = 0;

    for elf in elves {
        let calories: usize = elf
            .trim_end()
            .split('\n')
            .map(|s| str::parse(s).unwrap())
            .reduce(|accum, item| accum + item)
            .unwrap();

        highest = highest.max(calories);
    }

    println!("First solution: {highest:#?}");
}

fn read_file(filename: &str) -> Result<String> {
    fs::read_to_string(filename)
}
