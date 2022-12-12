use std::collections::HashSet;

use utils::{self, fs::DataType};

pub fn solver(contents: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut rope = vec![(0, 0); 10];

    let instructions: Vec<String> = contents.split('\n').map(String::from).collect();

    for line in instructions {
        let line: Vec<&str> = line.split(' ').collect();
        let direction = line[0];
        let amount = str::parse::<u32>(line[1]).unwrap();

        for _ in 0..amount {
            let dir = match direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };
            rope[0] = (dir.0 + rope[0].0, dir.1 + rope[0].1);

            for idx in 1..rope.len() {
                let head = rope[idx - 1];
                let tail = &mut rope[idx];
                let (x_diff, y_diff): (i32, i32) = (head.0 - tail.0, head.1 - tail.1);
                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    if tail.0 != head.0 && tail.1 != head.1 {
                        *tail = (
                            tail.0 + x_diff / x_diff.abs(),
                            tail.1 + y_diff / y_diff.abs(),
                        );
                    } else if x_diff.abs() > 1 {
                        *tail = (tail.0 + x_diff / x_diff.abs(), tail.1);
                    } else {
                        *tail = (tail.0, tail.1 + y_diff / y_diff.abs());
                    }
                }
            }

            visited.insert(rope[9]);
        }
    }

    let ans = visited.len();

    println!("Part two solution: {}", ans);

    ans
}

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    solver(&contents);
}

pub fn solver_one(contents: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut head = (0, 0);
    let mut tail = head;

    let instructions: Vec<String> = contents.split('\n').map(String::from).collect();

    for line in instructions {
        let line: Vec<&str> = line.split(' ').collect();
        let direction = line[0];
        let amount = str::parse::<u32>(line[1]).unwrap();

        for _ in 0..amount {
            let dir = match direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };

            head = (dir.0 + head.0, dir.1 + head.1);

            let (x_diff, y_diff): (i32, i32) = (head.0 - tail.0, head.1 - tail.1);
            if x_diff.abs() > 1 || y_diff.abs() > 1 {
                if tail.0 != head.0 && tail.1 != head.1 {
                    tail = (
                        tail.0 + x_diff / x_diff.abs(),
                        tail.1 + y_diff / y_diff.abs(),
                    );
                } else if x_diff.abs() > 1 {
                    tail = (tail.0 + x_diff / x_diff.abs(), tail.1);
                } else {
                    tail = (tail.0, tail.1 + y_diff / y_diff.abs());
                }

                visited.insert(tail);
            }
        }
    }

    let ans = visited.len();

    println!("Part one solution: {}", ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::{solver_one, solver};

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part_one_works() {
        let expected = 13;
        let actual = solver_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = 1;
        let actual = solver(INPUT);

        assert_eq!(actual, expected);
    }

    const INPUT_TWO: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part_two_example_two() {
        let expected = 36;
        let actual = solver(INPUT_TWO);

        assert_eq!(actual, expected);
    }
}
