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
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::part_one;

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
}
