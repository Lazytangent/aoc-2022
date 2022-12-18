use std::str::FromStr;

use utils::{self, cli::Part, fs::DataType};

pub fn solve(r#type: DataType, part: Part) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        Part::One => part_one(&contents),
        Part::Two => part_two(&contents),
    };

    println!("Part {part} solution: {val:?}");
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    sides_uncovered: u8,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Point, Self::Err> {
        let coords: Vec<i32> = s.split(',').map(|s| s.parse().unwrap()).collect();

        Ok(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            sides_uncovered: 6,
        })
    }
}

pub fn part_one(contents: &str) -> usize {
    let mut points: Vec<Point> = contents.split('\n').map(|l| l.parse().unwrap()).collect();

    for i in 1..points.len() {
        for j in (0..i).rev() {
            let mut curr = points[i].clone();
            let mut prev = points[j].clone();

            if curr.x == prev.x && curr.y == prev.y && (curr.z == prev.z - 1 || curr.z == prev.z + 1) {
                if curr.sides_uncovered > 0 {
                    curr.sides_uncovered -= 1;
                }
                if prev.sides_uncovered > 0 {
                    prev.sides_uncovered -= 1;
                }
            } else if curr.z == prev.z && curr.y == prev.y && (curr.x == prev.x - 1 || curr.x == prev.x + 1) {
                if curr.sides_uncovered > 0 {
                    curr.sides_uncovered -= 1;
                }
                if prev.sides_uncovered > 0 {
                    prev.sides_uncovered -= 1;
                }
            } else if curr.x == prev.x && curr.z == prev.z && (curr.y == prev.y - 1 || curr.y == prev.y + 1) {
                if curr.sides_uncovered > 0 {
                    curr.sides_uncovered -= 1;
                }
                if prev.sides_uncovered > 0 {
                    prev.sides_uncovered -= 1;
                }
            }

            points[i] = curr;
            points[j] = prev;
        }
    }

    points.iter().map(|p| p.sides_uncovered as usize).sum()
}

pub fn part_two(contents: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::part_one;

    const INPUT1: &str = "1,1,1
2,1,1";

    const INPUT2: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn one_input_one() {
        assert_eq!(part_one(INPUT1), 10);
    }

    #[test]
    fn one_input_two() {
        assert_eq!(part_one(INPUT2), 64);
    }
}
