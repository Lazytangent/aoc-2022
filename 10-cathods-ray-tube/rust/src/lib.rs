use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    match part {
        1 => {
            part_one(contents);
        }
        2 => part_two(contents),
        _ => unreachable!(),
    };
}

pub fn part_one(contents: String) -> Vec<i32> {
    let instructions: Vec<String> = contents.split('\n').map(String::from).collect();
    let mut reg_values: Vec<i32> = vec![1; 241];
    let mut idx: usize = 2;

    for line in instructions {
        reg_values[idx] = reg_values[idx - 1];
        idx += 1;

        if idx > 240 {
            break;
        }

        if line.starts_with("noop") {
            continue;
        }

        let line = line.split(' ').collect::<Vec<&str>>();
        let amount: i32 = line[1].parse().unwrap();

        reg_values[idx] = reg_values[idx - 1] + amount;
        idx += 1;

        if idx > 240 {
            break;
        }
    }

    let interesting_indicies = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    for i in interesting_indicies {
        sum += reg_values[i] * i as i32;
    }

    println!("Part one solution: {sum}");

    reg_values
}

pub fn part_two(contents: String) {
    let mut screen = vec![vec!["."; 40]; 6];

    let values = part_one(contents);
    let mut row = 0;
    let mut col = 0;

    for cycle in 1..values.len() {
        let val = values[cycle];

        if cycle > 1 && cycle % 40 == 1 {
            col = 0;
            row += 1;
        }

        if col == val || col == val - 1 || col == val + 1 {
            screen[row][col as usize] = "#";
        }

        col += 1;
    }

    println!("Part two solution:");
    print_screen(&screen);
}

fn print_screen(screen: &Vec<Vec<&str>>) {
    let mut contents = String::new();

    for row in screen {
        let row: String = row.iter().map(|s| s.to_string()).collect();
        contents.push_str(&row);
        contents.push('\n');
    }

    println!("{contents}");
}
