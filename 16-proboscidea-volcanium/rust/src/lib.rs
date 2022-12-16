use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        1 => part_one(&contents),
        2 => part_two(&contents),
        _ => unreachable!(),
    };

    println!("Part {part} solution: {val:?}");
}

pub fn part_one(contents: &str) {
    let valves: Vec<&str> = contents.split('\n').collect();
}

pub fn part_two(contents: &str) {}
