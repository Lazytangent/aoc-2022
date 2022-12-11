use std::{collections::HashSet, cmp::Reverse};

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    match part {
        1 => part_one(contents),
        2 => part_two(contents),
        _ => unreachable!(),
    };
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

pub fn part_one(contents: String) {
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

    println!("Part one solution: {}", monkeys[0].items_seen * monkeys[1].items_seen);
}

pub fn part_two(contents: String) {}
