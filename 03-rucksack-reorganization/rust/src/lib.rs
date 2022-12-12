use std::collections::HashSet;

use utils::fs::DataType;

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);
    let sum = part_two(&contents);

    println!("Second solution: {sum}");
}

pub fn part_two(contents: &str) -> usize {
    let rucksacks: Vec<String> = contents.split('\n').map(String::from).collect();

    let mut sum = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        let first: HashSet<char> = HashSet::from_iter(rucksacks[i].chars().collect::<Vec<char>>());
        let second: HashSet<char> =
            HashSet::from_iter(rucksacks[i + 1].chars().collect::<Vec<char>>());
        let third: HashSet<char> =
            HashSet::from_iter(rucksacks[i + 2].chars().collect::<Vec<char>>());

        let first_second: HashSet<_> = first.intersection(&second).map(|c| c.to_owned()).collect();
        let full = first_second.intersection(&third);
        let symbol = full.last().unwrap();

        if symbol.is_lowercase() {
            sum += (*symbol as u8 - 'a' as u8 + 1) as usize;
        } else {
            sum += (*symbol as u8 - 'A' as u8) as usize + 27;
        }
    }

    sum
}

pub fn part_one(contents: &str) -> usize {
    let rucksacks: Vec<String> = contents.split('\n').map(String::from).collect();

    let mut sum = 0;

    for rucksack in rucksacks {
        let n = rucksack.len() / 2;
        for c in rucksack[..n].chars() {
            if let Some(_) = rucksack[n..].find(c) {
                let priority = if c.is_lowercase() {
                    (c as u8 - 'a' as u8 + 1) as usize
                } else {
                    (c as u8 - 'A' as u8) as usize + 27
                };
                sum += priority;
                break;
            }
        }
    }

    sum
}

pub fn solve_one(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);
    let sum = part_one(&contents);

    println!("First solution: {sum}");
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one_works() {
        let expected = 157;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = 70;
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
