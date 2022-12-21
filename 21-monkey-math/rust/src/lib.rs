use std::collections::HashMap;

use utils::{self, cli::Part, fs::DataType};

pub fn solve(r#type: DataType, part: Part) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        Part::One => part_one(&contents),
        Part::Two => part_two(&contents),
    };

    println!("Part {part} solution: {val:?}");
}

pub fn part_one(contents: &str) -> i64 {
    let monkeys: Vec<&str> = contents.split('\n').collect();
    let mut required_values: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    let mut found_values: HashMap<&str, i64> = HashMap::new();

    for monkey in monkeys {
        let monkey: Vec<&str> = monkey.split(": ").collect();
        let name = monkey[0];
        let value = monkey[1];

        if let Ok(v) = value.parse::<i64>() {
            found_values.insert(name, v);
            continue;
        }

        let value: Vec<&str> = value.split(' ').collect();
        let (required_monkey_one, operation, required_monkey_two) = (value[0], value[1], value[2]);
        required_values.insert(name, (required_monkey_one, operation, required_monkey_two));
    }

    calculate_monkey_value(&mut found_values, &mut required_values, "root")
}

fn calculate_monkey_value<'a>(found: &mut HashMap<&'a str, i64>, required: &mut HashMap<&'a str, (&'a str, &'a str, &'a str)>, name: &'a str) -> i64 {
    if found.contains_key(&name) {
        return found[name];
    }

    assert!(required.contains_key(&name));

    let (first, operation, second) = required[name];
    let first = calculate_monkey_value(found, required, first);
    let second = calculate_monkey_value(found, required, second);

    let value = match operation {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" => first / second,
        _ => unreachable!(),
    };

    found.insert(name, value);

    value
}

pub fn part_two(contents: &str) -> i64 {
    let monkeys: Vec<&str> = contents.split('\n').collect();
    let mut required_values: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    let mut found_values: HashMap<&str, i64> = HashMap::new();

    for monkey in monkeys {
        let monkey: Vec<&str> = monkey.split(": ").collect();
        let name = monkey[0];
        let value = monkey[1];

        if let Ok(v) = value.parse::<i64>() {
            found_values.insert(name, v);
            continue;
        }

        let value: Vec<&str> = value.split(' ').collect();
        let (required_monkey_one, operation, required_monkey_two) = (value[0], value[1], value[2]);
        required_values.insert(name, (required_monkey_one, operation, required_monkey_two));
    }

    let mut human_path = find_human_path(&found_values, &required_values, "root").unwrap();
    human_path.pop();

    let (left, _, right) = required_values["root"];
    if left == &human_path.pop().unwrap() {
        let right_res = calculate_monkey_value(&mut found_values, &mut required_values, right);
        calculate_value(&mut found_values, &mut required_values, &mut human_path, left, right_res)
    } else {
        let left_res = calculate_monkey_value(&mut found_values, &mut required_values, left);
        calculate_value(&mut found_values, &mut required_values, &mut human_path, right, left_res)
    }
}

fn find_human_path(found: &HashMap<&str, i64>, required: &HashMap<&str, (&str, &str, &str)>, name: &str) -> Option<Vec<String>> {
    if name == "humn" {
        return Some(vec![name.to_string()]);
    }

    if found.contains_key(name) {
        None
    } else {
        assert!(required.contains_key(name));
        let (m1, _, m2) = required[name];

        if let Some(mut v) = find_human_path(found, required, m1) {
            v.push(name.to_string());
            Some(v)
        } else if let Some(mut v) = find_human_path(found, required, m2) {
            v.push(name.to_string());
            Some(v)
        } else {
            None
        }
    }
}

fn calculate_value<'a>(found: &mut HashMap<&'a str, i64>, required: &mut HashMap<&'a str, (&'a str, &'a str, &'a str)>, human_path: &mut Vec<String>, name: &'a str, equal_to: i64) -> i64 {
    if name == "humn" {
        return equal_to;
    }
    if found.contains_key(name) {
        return found[name];
    }

    assert!(required.contains_key(name));

    let (left, op, right) = required[name];
    let is_human_left = left == &human_path.pop().unwrap();
    let (to_solve, other) = if is_human_left {
        (left, calculate_monkey_value(found, required, right))
    } else {
        (right, calculate_monkey_value(found, required, left))
    };

    match (op, is_human_left) {
        ("+", _) => calculate_value(found, required, human_path, to_solve, equal_to - other),
        ("-", true) => calculate_value(found, required, human_path, to_solve, equal_to + other),
        ("-", false) => calculate_value(found, required, human_path, to_solve, other - equal_to),
        ("*", _) => calculate_value(found, required, human_path, to_solve, equal_to / other),
        ("/", true) => calculate_value(found, required, human_path, to_solve, equal_to * other),
        ("/", false) => calculate_value(found, required, human_path, to_solve, other / equal_to),
        (_, _) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 152);
    }

    #[test]
    fn two() {
        assert_eq!(part_two(INPUT), 301);
    }
}
