use std::collections::HashSet;

use utils::{self, fs::DataType};

pub fn solver(contents: String) {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut rope = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    let adjacent = [
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (0, 0),
    ];

    let mut insertions = vec![];

    let instructions: Vec<String> = contents.split('\n').map(String::from).collect();

    for line in instructions {
        let line: Vec<&str> = line.split(' ').collect();
        let direction = line[0];
        let amount = str::parse::<u32>(line[1]).unwrap();

        for _ in 0..amount {
            let mut prev = rope[0];

            match direction {
                "U" => {
                    rope[0] = (rope[0].0, rope[0].1 + 1);
                }
                "D" => {
                    rope[0] = (rope[0].0, rope[0].1 - 1);
                }
                "L" => {
                    rope[0] = (rope[0].0 - 1, rope[0].1);
                }
                "R" => {
                    rope[0] = (rope[0].0 + 1, rope[0].1);
                }
                _ => ()
            };

            for idx in 1..rope.len() {
                let mut move_node = true;
                let node = rope[idx];

                for dir in adjacent {
                    let location = (node.0 + dir.0, node.1 + dir.1);

                    if location == rope[idx - 1] {
                        move_node = false;
                        break;
                    }
                }

                if move_node {
                    rope[idx] = prev;

                    prev = node;
                    if idx == 9 {
                        visited.insert(rope[idx]);
                        insertions.push(rope[idx]);
                    }
                } else {
                    break;
                }
            }
        }
    }

    println!("Insertions in order: {insertions:#?}");

    println!("Part two solution: {}", visited.len());
}

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    solver(contents);
}

pub fn solve_one(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut head = (0, 0);
    let mut tail = head;

    let adjacent = [
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (0, 0),
    ];

    let instructions: Vec<String> = contents.split('\n').map(String::from).collect();

    for line in instructions {
        let line: Vec<&str> = line.split(' ').collect();
        let direction = line[0];
        let amount = str::parse::<u32>(line[1]).unwrap();

        for _ in 0..amount {
            let prev = head;
            let mut move_tail = true;

            match direction {
                "U" => {
                    head = (head.0, head.1 + 1);
                }
                "D" => {
                    head = (head.0, head.1 - 1);
                }
                "L" => {
                    head = (head.0 - 1, head.1);
                }
                "R" => {
                    head = (head.0 + 1, head.1);
                }
                _ => ()
            };

            for dir in adjacent {
                let location = (tail.0 + dir.0, tail.1 + dir.1);

                if location == head {
                    move_tail = false;
                    break;
                }
            }

            if move_tail {
                tail = prev;
                visited.insert(tail);
            }
        }
    }

    println!("Part one solution: {}", visited.len());
}

#[cfg(test)]
mod tests {
    use super::solver;

    const SAMPLE_TWO: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_sample_input_two() {
        let contents = SAMPLE_TWO.to_string();

        solver(contents);
    }
}
