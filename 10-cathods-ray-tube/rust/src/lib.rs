use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    match part {
        1 => {
            let val = part_one(&contents);
            println!("Part one solution: {val}");
        }
        2 => {
            let screen = part_two(&contents);
            let screen = print_screen(&screen);
            println!("Part two solution:");
            println!("{screen}");
        }
        _ => unreachable!(),
    };
}

pub fn part_one(contents: &str) -> i32 {
    let reg_values = calculate_values(contents);
    let interesting_indicies = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    for i in interesting_indicies {
        sum += reg_values[i] * i as i32;
    }

    sum
}

fn calculate_values(contents: &str) -> Vec<i32> {
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

    reg_values
}

pub fn part_two(contents: &str) -> Vec<Vec<&str>> {
    let mut screen = vec![vec!["."; 40]; 6];

    let values = calculate_values(contents);
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

    screen
}

pub fn print_screen(screen: &Vec<Vec<&str>>) -> String {
    let mut contents = String::new();

    for row in screen {
        let row: String = row.iter().map(|s| s.to_string()).collect();
        contents.push_str(&row);
        contents.push('\n');
    }

    contents
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two, print_screen};

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part_one_works() {
        let expected = 13140;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    const EXPECTED: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    #[test]
    fn part_two_works() {
        let grid = part_two(INPUT);
        let actual = print_screen(&grid);

        assert_eq!(&actual, EXPECTED);
    }
}
