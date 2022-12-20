use std::collections::VecDeque;

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
    let numbers: Vec<i64> = contents.split('\n').map(|s| s.parse().unwrap()).collect();
    let mut positions: VecDeque<usize> = (0..numbers.len()).collect();
    let wrap = positions.len() - 1;

    mix(&numbers, &mut positions, wrap);

    calculate_coordinates(&numbers, positions)
}

fn mix(data: &Vec<i64>, positions: &mut VecDeque<usize>, wrap: usize) {
    for (idx, val) in data.iter().enumerate() {
        if *val == 0 {
            continue;
        }

        let pos = positions.iter().position(|x| *x == idx).unwrap();
        let tmp = positions.remove(pos).unwrap();

        if *val < 0 {
            let val = val.unsigned_abs() as usize % wrap;
            positions.rotate_right(val);
            positions.insert(pos, tmp);
        } else {
            let val = val.unsigned_abs() as usize % wrap;
            positions.rotate_left(val);
            positions.insert(pos, tmp);
        }
    }
}

fn calculate_coordinates(data: &Vec<i64>, positions: VecDeque<usize>) -> i64 {
    let original_idx_of_zero = data.iter().position(|x| *x == 0).unwrap();
    let idx_of_zero = positions.iter().position(|x| *x == original_idx_of_zero).unwrap();
    (1..=3).map(|i| {
        let search_pos = (i * 1000 + idx_of_zero) % positions.len();
        data[positions[search_pos]]
    })
    .sum()
}

pub fn part_two(contents: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::part_one;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 3);
    }
}
