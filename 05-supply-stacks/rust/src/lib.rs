use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let halves: Vec<String> = contents.split("\n\n").map(String::from).collect();
    let drawing: Vec<String> = halves[0].split('\n').map(String::from).collect();
    let moves: Vec<String> = halves[1].split('\n').map(String::from).collect();

    let mut count = 1;
    let mut spaces = 1;
    let stack_row = &drawing[drawing.len() - 1];

    while spaces + 4 < stack_row.len() {
        spaces += 4;
        count += 1;
    }

    let mut stacks: Vec<Vec<char>> = vec![vec![]; count];

    for i in (0..drawing.len() - 1).rev() {
        let row = drawing[i].as_bytes();
        let mut idx = 1;
        let mut stack = 0;

        while idx <= spaces {
            if row.len() > idx {
                if row[idx] != ' ' as u8 {
                    stacks[stack].push(row[idx] as char);
                }
            }

            idx += 4;
            stack += 1;
        }
    }

    for m in moves {
        let m: Vec<String> = m.split_whitespace().map(String::from).collect();
        let how_many: usize = str::parse(&m[1]).unwrap();
        let from: usize = str::parse(&m[3]).unwrap();
        let to: usize = str::parse(&m[5]).unwrap();

        let stack_from = &mut stacks[from - 1];
        let moved_slice: Vec<char> = stack_from.drain((stack_from.len() - how_many)..).collect();
        stacks[to - 1].extend_from_slice(&moved_slice);
    }

    let last_items: String = stacks.into_iter().map(|mut s| s.pop().unwrap()).collect();
    println!("Part two solution: {last_items}");
}

pub fn part_two(contents: &str) -> String {
    let halves: Vec<String> = contents.split("\n\n").map(String::from).collect();
    let drawing: Vec<String> = halves[0].split('\n').map(String::from).collect();
    let moves: Vec<String> = halves[1].split('\n').map(String::from).collect();

    let mut count = 1;
    let mut spaces = 1;
    let stack_row = &drawing[drawing.len() - 1];

    while spaces + 4 < stack_row.len() {
        spaces += 4;
        count += 1;
    }

    let mut stacks: Vec<Vec<char>> = vec![vec![]; count];

    for i in (0..drawing.len() - 1).rev() {
        let row = drawing[i].as_bytes();
        let mut idx = 1;
        let mut stack = 0;

        while idx <= spaces {
            if row.len() > idx {
                if row[idx] != ' ' as u8 {
                    stacks[stack].push(row[idx] as char);
                }
            }

            idx += 4;
            stack += 1;
        }
    }

    for m in moves {
        let m: Vec<String> = m.split_whitespace().map(String::from).collect();
        let how_many: usize = str::parse(&m[1]).unwrap();
        let from: usize = str::parse(&m[3]).unwrap();
        let to: usize = str::parse(&m[5]).unwrap();

        let stack_from = &mut stacks[from - 1];
        let moved_slice: Vec<char> = stack_from.drain((stack_from.len() - how_many)..).collect();
        stacks[to - 1].extend_from_slice(&moved_slice);
    }

    stacks.into_iter().map(|mut s| s.pop().unwrap()).collect()
}

pub fn part_one(contents: &str) -> String {
    let halves: Vec<String> = contents.split("\n\n").map(String::from).collect();
    let drawing: Vec<String> = halves[0].split('\n').map(String::from).collect();
    let moves: Vec<String> = halves[1].split('\n').map(String::from).collect();

    let mut count = 1;
    let mut spaces = 1;
    let stack_row = &drawing[drawing.len() - 1];

    while spaces + 4 < stack_row.len() {
        spaces += 4;
        count += 1;
    }

    let mut stacks: Vec<Vec<char>> = vec![vec![]; count];

    for i in (0..drawing.len() - 1).rev() {
        let row = drawing[i].as_bytes();
        let mut idx = 1;
        let mut stack = 0;

        while idx <= spaces {
            if row.len() > idx {
                if row[idx] != ' ' as u8 {
                    stacks[stack].push(row[idx] as char);
                }
            }

            idx += 4;
            stack += 1;
        }
    }

    for m in moves {
        let m: Vec<String> = m.split_whitespace().map(String::from).collect();
        let how_many: usize = str::parse(&m[1]).unwrap();
        let from: usize = str::parse(&m[3]).unwrap();
        let to: usize = str::parse(&m[5]).unwrap();

        for _ in 0..how_many {
            let last = stacks[from - 1].pop();
            if let Some(v) = last {
                stacks[to - 1].push(v);
            }
        }
    }

    stacks.into_iter().map(|mut s| s.pop().unwrap()).collect()
}

pub fn solve_one(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let last_items = part_one(&contents);
    println!("Part one solution: {last_items}");
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_one_works() {
        let expected = "CMZ";
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = "MCD";
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
