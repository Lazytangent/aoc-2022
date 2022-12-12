use std::cmp::Reverse;

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        1 => part_one(&contents),
        2 => part_two(&contents),
        _ => unreachable!(),
    };

    println!("Part {part} solution: {val}");
}

#[derive(Debug, Clone)]
struct Monkey {
    items_seen: usize,
    operation: String,
    conditional: usize,
    r#true: usize,
    r#false: usize,
    no: usize,
}

enum Operation {
    Multiplication,
    Addition,
}

enum OtherValue {
    Old,
    Int(usize),
}

pub fn part_one(contents: &str) -> usize {
    let monkey_instructions: Vec<String> = contents.split("\n\n").map(String::from).collect();
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(monkey_instructions.len());
    let mut items: Vec<Vec<usize>> = vec![vec![]; monkey_instructions.len()];

    for (idx, monkey) in monkey_instructions.iter().enumerate() {
        let instructions: Vec<&str> = monkey.split('\n').collect();
        let line_one: Vec<&str> = instructions[1].split(": ").collect();
        let starting_items: Vec<usize> = line_one[1]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        items[idx] = starting_items;

        let line_two: Vec<&str> = instructions[2].split(": ").collect();
        let operation: &str = line_two[1];

        let line_three: Vec<&str> = instructions[3].split(": ").collect();
        let conditional: Vec<&str> = line_three[1].split(' ').collect();
        let conditional_number: usize = conditional[2].parse().unwrap();

        let line_four: Vec<&str> = instructions[4].trim().split(' ').collect();
        let cond_true: usize = line_four[5].parse().unwrap();

        let line_five: Vec<&str> = instructions[5].trim().split(' ').collect();
        let cond_false: usize = line_five[5].parse().unwrap();

        monkeys.push(Monkey {
            items_seen: 0,
            operation: operation.to_string(),
            conditional: conditional_number,
            r#true: cond_true,
            r#false: cond_false,
            no: idx,
        });
    }

    for _ in 0..20 {
        for monkey in &mut monkeys {
            let operation: Vec<&str> = monkey.operation.split(' ').collect();

            let operator = if operation[3] == "*" {
                Operation::Multiplication
            } else {
                Operation::Addition
            };

            let other_val = if operation[4] == "old" {
                OtherValue::Old
            } else {
                let val: usize = operation[4].parse().unwrap();
                OtherValue::Int(val)
            };

            while !items[monkey.no].is_empty() {
                let item = items[monkey.no].pop().unwrap();
                monkey.items_seen += 1;

                let new_val = match (&operator, &other_val) {
                    (Operation::Multiplication, OtherValue::Old) => item * item,
                    (Operation::Multiplication, OtherValue::Int(v)) => item * v,
                    (Operation::Addition, OtherValue::Old) => item + item,
                    (Operation::Addition, OtherValue::Int(v)) => item + v,
                };

                let new_val = new_val / 3;

                if new_val % monkey.conditional == 0 {
                    items[monkey.r#true].push(new_val);
                } else {
                    items[monkey.r#false].push(new_val);
                }
            }
        }
    }

    monkeys.sort_by_key(|a| Reverse(a.items_seen));

    monkeys[0].items_seen * monkeys[1].items_seen
}

pub fn part_two(contents: &str) -> usize {
    let monkey_instructions: Vec<String> = contents.split("\n\n").map(String::from).collect();
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(monkey_instructions.len());
    let mut items: Vec<Vec<usize>> = vec![vec![]; monkey_instructions.len()];

    for (idx, monkey) in monkey_instructions.iter().enumerate() {
        let instructions: Vec<&str> = monkey.split('\n').collect();
        let line_one: Vec<&str> = instructions[1].split(": ").collect();
        let starting_items: Vec<usize> = line_one[1]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        items[idx] = starting_items;

        let line_two: Vec<&str> = instructions[2].split(": ").collect();
        let operation: &str = line_two[1];

        let line_three: Vec<&str> = instructions[3].split(": ").collect();
        let conditional: Vec<&str> = line_three[1].split(' ').collect();
        let conditional_number: usize = conditional[2].parse().unwrap();

        let line_four: Vec<&str> = instructions[4].trim().split(' ').collect();
        let cond_true: usize = line_four[5].parse().unwrap();

        let line_five: Vec<&str> = instructions[5].trim().split(' ').collect();
        let cond_false: usize = line_five[5].parse().unwrap();

        monkeys.push(Monkey {
            items_seen: 0,
            operation: operation.to_string(),
            conditional: conditional_number,
            r#true: cond_true,
            r#false: cond_false,
            no: idx,
        });
    }

    let multiple: usize = monkeys.iter().map(|m| m.conditional).product();

    for _ in 0..10000 {
        for monkey in &mut monkeys {
            let operation: Vec<&str> = monkey.operation.split(' ').collect();

            let operator = if operation[3] == "*" {
                Operation::Multiplication
            } else {
                Operation::Addition
            };

            let other_val = if operation[4] == "old" {
                OtherValue::Old
            } else {
                let val: usize = operation[4].parse().unwrap();
                OtherValue::Int(val)
            };

            while !items[monkey.no].is_empty() {
                let item = items[monkey.no].pop().unwrap();
                monkey.items_seen += 1;

                let new_val = match (&operator, &other_val) {
                    (Operation::Multiplication, OtherValue::Old) => item * item,
                    (Operation::Multiplication, OtherValue::Int(v)) => item * v,
                    (Operation::Addition, OtherValue::Old) => item + item,
                    (Operation::Addition, OtherValue::Int(v)) => item + v,
                };

                let new_val = new_val % multiple;

                if new_val % monkey.conditional == 0 {
                    items[monkey.r#true].push(new_val);
                } else {
                    items[monkey.r#false].push(new_val);
                }
            }
        }
    }

    monkeys.sort_by_key(|a| Reverse(a.items_seen));

    monkeys[0].items_seen * monkeys[1].items_seen
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part_one_works() {
        let expected = 10605;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = 2713310158;
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
