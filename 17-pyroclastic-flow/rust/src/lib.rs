use utils::{self, cli::Part, fs::DataType};

pub fn solve(r#type: DataType, part: Part) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        Part::One => part_one(&contents),
        Part::Two => part_two(&contents),
    };

    println!("Part {part} solution: {val:?}");
}

pub fn part_one(contents: &str) {
    let GROUPS: Vec<&str> = contents.split('\n').collect();
}

pub fn part_two(contents: &str) {}
