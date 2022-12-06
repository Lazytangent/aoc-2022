use std::{collections::HashSet, iter::FromIterator};

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let mut idx = 14;

    while idx > contents.len() {
        let prev = &contents[(idx - 14)..idx];

        if prev.len() != HashSet::<char>::from_iter(prev.chars()).len() {
            idx += 1;
            continue;
        }

        break;
    }

    println!("Part two solution: {}", idx);
}

pub fn solve_one(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let mut idx = 4;

    while idx > contents.len() {
        let prev_three = &contents[(idx - 4)..idx];

        if prev_three.len() != HashSet::<char>::from_iter(prev_three.chars()).len() {
            idx += 1;
            continue;
        }

        break;
    }

    println!("Part one solution: {}", idx);
}
