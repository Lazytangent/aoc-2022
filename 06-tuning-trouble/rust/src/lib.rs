use std::{collections::HashSet, iter::FromIterator};

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let idx = solver(&contents, 14);

    println!("Part two solution: {}", idx);
}

pub fn solver(contents: &str, length: usize) -> usize {
    let mut idx = length;

    while idx < contents.len() {
        let prev = &contents[(idx - length)..idx];

        if prev.len() != HashSet::<char>::from_iter(prev.chars()).len() {
            idx += 1;
            continue;
        }

        break;
    }

    idx
}

pub fn solve_one(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let idx = solver(&contents, 4);

    println!("Part one solution: {}", idx);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::solver;

    const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[rstest]
    #[case(INPUT1, 7)]
    #[case(INPUT2, 5)]
    #[case(INPUT3, 6)]
    #[case(INPUT4, 10)]
    #[case(INPUT5, 11)]
    fn part_one(#[case] input: &str, #[case] expected: usize) {
        let actual = solver(input, 4);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(INPUT1, 19)]
    #[case(INPUT2, 23)]
    #[case(INPUT3, 23)]
    #[case(INPUT4, 29)]
    #[case(INPUT5, 26)]
    fn part_two(#[case] input: &str, #[case] expected: usize) {
        let actual = solver(input, 14);

        assert_eq!(actual, expected);
    }
}
