mod trees;

use utils::{self, fs::DataType};

use crate::trees::Trees;

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let lines: Vec<String> = contents.split('\n').map(String::from).collect();
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        matrix.push(line.chars().filter_map(|x| x.to_digit(10)).collect());
    }

    let mut trees = Trees::new(matrix);
    trees.find_all_visible_trees();

    println!("Part one solution: {}", trees.visible.len());

    println!("Part two solution: {}", trees.find_best_scenic_score());
}
