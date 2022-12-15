use std::collections::HashSet;

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        1 => {
            match r#type {
                DataType::Sample => part_one(&contents, 10),
                DataType::Real => part_one(&contents, 2_000_000),
                _ => unreachable!(),
            }
        }
        2 => {
            match r#type {
                DataType::Sample => part_two(&contents, 20, r#type),
                DataType::Real => part_two(&contents, 4_000_000, r#type),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };

    println!("Part {part} solution: {val:?}");
}

pub fn part_one(contents: &str, y_row: i32) -> usize {
    let positions: Vec<&str> = contents.split('\n').collect();
    let mut set: HashSet<Coordinate> = HashSet::new();

    for position in positions {
        let position: Vec<&str> = position.split(": ").collect();
        let sensor_data: Vec<&str> = position[0].split(' ').collect();
        let beacon_data: Vec<&str> = position[1].split(' ').collect();

        let sensor_x_data: Vec<&str> = sensor_data[2].trim_end_matches(',').split('=').collect();
        let sensor_y_data: Vec<&str> = sensor_data[3].split('=').collect();
        let sensor_x: i32 = sensor_x_data[1].parse().unwrap();
        let sensor_y: i32 = sensor_y_data[1].parse().unwrap();

        let beacon_x_data: Vec<&str> = beacon_data[4].trim_end_matches(',').split('=').collect();
        let beacon_y_data: Vec<&str> = beacon_data[5].split('=').collect();
        let beacon_x: i32 = beacon_x_data[1].parse().unwrap();
        let beacon_y: i32 = beacon_y_data[1].parse().unwrap();

        let sensor = Coordinate::new(sensor_x, sensor_y);
        let beacon = Coordinate::new(beacon_x, beacon_y);

        for c in sensor.coords_on_y_row(&beacon, y_row) {
            set.insert(c);
        }

        set.remove(&beacon);
    }

    set.len()
}

pub fn part_two(contents: &str, max_y: i32, r#type: DataType) -> usize {
    let positions: Vec<&str> = contents.split('\n').collect();

    match r#type {
        DataType::Sample => {
            let mut grid = SmallGrid::new();
            for position in positions {
                let position: Vec<&str> = position.split(": ").collect();
                let sensor_data: Vec<&str> = position[0].split(' ').collect();
                let beacon_data: Vec<&str> = position[1].split(' ').collect();

                let sensor_x_data: Vec<&str> = sensor_data[2].trim_end_matches(',').split('=').collect();
                let sensor_y_data: Vec<&str> = sensor_data[3].split('=').collect();
                let sensor_x: i32 = sensor_x_data[1].parse().unwrap();
                let sensor_y: i32 = sensor_y_data[1].parse().unwrap();

                let beacon_x_data: Vec<&str> = beacon_data[4].trim_end_matches(',').split('=').collect();
                let beacon_y_data: Vec<&str> = beacon_data[5].split('=').collect();
                let beacon_x: i32 = beacon_x_data[1].parse().unwrap();
                let beacon_y: i32 = beacon_y_data[1].parse().unwrap();

                let sensor = Coordinate::new(sensor_x, sensor_y);
                let beacon = Coordinate::new(beacon_x, beacon_y);

                grid.add_sensor_pair(&sensor, &beacon);
            }

            let c = grid.find_empty();
            (c.x * 4_000_000 + c.y) as usize
        }
        DataType::Real => {
            let mut grid = Grid::new();
            for position in positions {
                let position: Vec<&str> = position.split(": ").collect();
                let sensor_data: Vec<&str> = position[0].split(' ').collect();
                let beacon_data: Vec<&str> = position[1].split(' ').collect();

                let sensor_x_data: Vec<&str> = sensor_data[2].trim_end_matches(',').split('=').collect();
                let sensor_y_data: Vec<&str> = sensor_data[3].split('=').collect();
                let sensor_x: i32 = sensor_x_data[1].parse().unwrap();
                let sensor_y: i32 = sensor_y_data[1].parse().unwrap();

                let beacon_x_data: Vec<&str> = beacon_data[4].trim_end_matches(',').split('=').collect();
                let beacon_y_data: Vec<&str> = beacon_data[5].split('=').collect();
                let beacon_x: i32 = beacon_x_data[1].parse().unwrap();
                let beacon_y: i32 = beacon_y_data[1].parse().unwrap();

                let sensor = Coordinate::new(sensor_x, sensor_y);
                let beacon = Coordinate::new(beacon_x, beacon_y);

                grid.add_sensor_pair(&sensor, &beacon);
            }

            let c = grid.find_empty();
            (c.x * 4_000_000 + c.y) as usize
        }
        _ => unreachable!(),
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn calculate_manhattan_distance(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn greatest_possible_y(&self, other: &Coordinate) -> i32 {
        self.y + self.calculate_manhattan_distance(&other)
    }

    fn smallest_possible_y(&self, other: &Coordinate) -> i32 {
        self.y - self.calculate_manhattan_distance(&other)
    }

    fn crosses_y(&self, other: &Coordinate, y: i32) -> bool {
        let y2 = self.greatest_possible_y(other);
        let y3 = self.smallest_possible_y(other);

        (y >= self.y && y <= y2) || (y >= y3 && y <= self.y)
    }

    fn coords_on_y_row(&self, other: &Coordinate, y_row: i32) -> HashSet<Coordinate> {
        let mut set: HashSet<Coordinate> = HashSet::new();
        let manhattan_dist = self.calculate_manhattan_distance(other);

        let y_diff = (y_row - self.y).abs();
        let potential_x_diff = manhattan_dist - y_diff;

        let x0 = self.x - potential_x_diff;
        let x1 = self.x + potential_x_diff;

        for x in x0..=x1 {
            set.insert(Coordinate::new(x, y_row));
        }

        set
    }
}

const SMALL_MAX: usize = 20;
const MAX: usize = 4_000_000;

#[derive(Debug)]
struct Grid {
    matrix: [[bool; MAX + 1]; MAX + 1],
}

#[derive(Debug)]
struct SmallGrid {
    matrix: [[bool; SMALL_MAX + 1]; SMALL_MAX + 1],
}

impl Grid {
    fn new() -> Self {
        Self {
            matrix: [[true; MAX + 1]; MAX + 1],
        }
    }

    fn add_sensor_pair(&mut self, sensor: &Coordinate, beacon: &Coordinate) {
        for y in 0..self.matrix.len() {
            for c in sensor.coords_on_y_row(beacon, y as i32) {
                if c.x >= 0 && c.x <= MAX as i32 && c.y >= 0 && c.y <= MAX as i32 {
                    self.matrix[c.x as usize][c.y as usize] = false;
                }
            }
        }
    }

    fn find_empty(&self) -> Coordinate {
        for x in 0..self.matrix.len() {
            for y in 0..self.matrix.len() {
                if self.matrix[x][y] {
                    return Coordinate::new(x as i32, y as i32);
                }
            }
        }

        unreachable!()
    }
}

impl SmallGrid {
    fn new() -> Self {
        Self {
            matrix: [[true; SMALL_MAX + 1]; SMALL_MAX + 1],
        }
    }

    fn add_sensor_pair(&mut self, sensor: &Coordinate, beacon: &Coordinate) {
        for y in 0..=SMALL_MAX {
            for c in sensor.coords_on_y_row(beacon, y as i32) {
                if c.x >= 0 && c.x <= SMALL_MAX as i32 && c.y >= 0 && c.y <= SMALL_MAX as i32 {
                    self.matrix[c.x as usize][c.y as usize] = false;
                }
            }
        }
    }

    fn find_empty(&self) -> Coordinate {
        for x in 0..self.matrix.len() {
            for y in 0..self.matrix.len() {
                if self.matrix[x][y] {
                    return Coordinate::new(x as i32, y as i32);
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use utils::fs::DataType;
    use super::{part_one, part_two};

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT, 10), 26);
    }

    #[test]
    fn two() {
        assert_eq!(part_two(INPUT, 20, DataType::Sample), 56_000_011);
    }
}
