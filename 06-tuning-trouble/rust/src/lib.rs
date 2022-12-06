use std::{collections::HashSet, iter::FromIterator};

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let mut idx = 13;

    loop {
        if idx > contents.len() {
            break;
        }

        let prev = &contents[(idx - 13)..idx];

        if prev.len() != HashSet::<char>::from_iter(prev.chars()).len() {
            idx += 1;
            continue;
        }

        if prev.contains(contents.as_bytes()[idx] as char) {
            idx += 1;
            continue;
        }

        break;
    }

    println!("Part two solution: {}", idx + 1);
}

pub fn solve_one(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let mut idx = 3;

    loop {
        if idx > contents.len() {
            break;
        }

        let prev_three = &contents[(idx - 3)..idx];

        if prev_three.len() != HashSet::<char>::from_iter(prev_three.chars()).len() {
            idx += 1;
            continue;
        }

        if contents[(idx - 3)..idx].contains(contents.as_bytes()[idx] as char) {
            idx += 1;
            continue;
        }

        break;
    }

    println!("Part one solution: {}", idx + 1);
}
