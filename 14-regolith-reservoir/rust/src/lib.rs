mod grid;

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        1 => part_one(&contents),
        2 => part_two(&contents),
        _ => unreachable!(),
    };

    println!("Part {part} solution: {val:?}");
}

pub fn part_one(contents: &str) -> usize {
    let mut map = grid::Grid::new();
    map.put(500, 0, '.');
    map.read(contents);

    let mut sand = 0;
    while map.drop(500, 0, map.ymax) {
        sand += 1;
    }

    sand
}

pub fn part_two(contents: &str) -> usize {
    let mut map = grid::Grid::new();
    map.put(500, 0, '.');
    map.read(contents);

    let mut sand = 0;
    while map.grid[&(500, 0)] == '.' {
        map.drop(500, 0, map.floor);
        sand += 1;
    }

    sand
}
