use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    match part {
        1 => part_one(contents),
        2 => part_two(contents),
        _ => unreachable!(),
    };
}

pub fn part_one(contents: String) {
    let GROUPS: Vec<String> = contents.split('\n').map(String::from).collect();
}

pub fn part_two(contents: String) {}
