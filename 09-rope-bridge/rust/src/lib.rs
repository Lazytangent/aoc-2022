use std::collections::HashSet;

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
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
