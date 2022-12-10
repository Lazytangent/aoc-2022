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
    let instructions: Vec<String> = contents.split('\n').map(String::from).collect();
    let mut reg_values: Vec<i32> = vec![1; 222];
    let mut idx: usize = 2;

    for line in instructions {
        reg_values[idx] = reg_values[idx - 1];
        idx += 1;

        if idx > 221 {
            break;
        }

        if line.starts_with("noop") {
            continue;
        }

        let line = line.split(' ').collect::<Vec<&str>>();
        let amount: i32 = line[1].parse().unwrap();

        reg_values[idx] = reg_values[idx - 1] + amount;
        idx += 1;

        if idx > 221 {
            break;
        }
    }

    let interesting_indicies = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    for i in interesting_indicies {
        sum += reg_values[i] * i as i32;
    }

    println!("Part one solution: {sum}");
}

pub fn part_two(contents: String) {}
