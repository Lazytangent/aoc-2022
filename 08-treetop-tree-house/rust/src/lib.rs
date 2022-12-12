mod trees;

use utils::{self, fs::DataType};

use crate::trees::Trees;

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let part_one_solution = part_one(&contents);
    let part_two_solution = part_two(&contents);

    println!("Part one solution: {}", part_one_solution);
    println!("Part two solution: {}", part_two_solution);
}

pub fn part_one(contents: &str) -> usize {
    let lines: Vec<String> = contents.split('\n').map(String::from).collect();
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        matrix.push(line.chars().filter_map(|x| x.to_digit(10)).collect());
    }

    let mut trees = Trees::new(matrix);
    trees.find_all_visible_trees();

    trees.visible.len()
}

pub fn part_two(contents: &str) -> usize {
    let lines: Vec<String> = contents.split('\n').map(String::from).collect();
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        matrix.push(line.chars().filter_map(|x| x.to_digit(10)).collect());
    }

    let mut trees = Trees::new(matrix);
    trees.find_all_visible_trees();

    trees.find_best_scenic_score()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part_one_works() {
        let expected = 21;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = 8;
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
