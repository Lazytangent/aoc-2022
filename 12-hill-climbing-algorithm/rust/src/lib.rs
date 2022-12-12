use std::collections::VecDeque;

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        1 => part_one(&contents),
        2 => part_two(&contents),
        _ => unreachable!(),
    };

    println!("Part {part} solution: {val}");
}

pub fn part_one(contents: &str) -> usize {
    let map: Vec<Vec<char>> = contents.split('\n').map(|s| s.chars().collect()).collect();
    let height: usize = map.len();
    let width: usize = map[0].len();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut grid: Vec<Vec<usize>> = vec![vec![0; width]; height];

    for i in 0..height {
        for j in 0..width {
            if map[i][j] == 'S' {
                start = (i, j);
            } else if map[i][j] == 'E' {
                end = (i, j);
                grid[i][j] = 26;
            } else {
                grid[i][j] = map[i][j] as usize - 'a' as usize;
            }
        }
    }

    let mut distances: Vec<Vec<usize>> = vec![vec![0; width]; height];
    distances[start.0][start.1] = 0;
    let mut seen: VecDeque<(usize, usize)> = VecDeque::new();
    seen.push_back(start);

    while let Some(point) = seen.pop_front() {
        let steps = distances[point.0][point.1];

        if point.0 + 1 < height {
            let new = (point.0 + 1, point.1);
            if (distances[new.0][new.1] == 0 && new != start) || steps + 1 < distances[new.0][new.1] {
                if grid[new.0][new.1] <= grid[point.0][point.1] + 1 {
                    distances[new.0][new.1] = steps + 1;
                    seen.push_back(new);
                }
            }
        }
        if point.0 > 0 {
            let new = (point.0 - 1, point.1);
            if (distances[new.0][new.1] == 0 && new != start) || steps + 1 < distances[new.0][new.1] {
                if grid[new.0][new.1] <= grid[point.0][point.1] + 1 {
                    distances[new.0][new.1] = steps + 1;
                    seen.push_back(new);
                }
            }
        }
        if point.1 + 1 < width {
            let new = (point.0, point.1 + 1);
            if (distances[new.0][new.1] == 0 && new != start) || steps + 1 < distances[new.0][new.1] {
                if grid[new.0][new.1] <= grid[point.0][point.1] + 1 {
                    distances[new.0][new.1] = steps + 1;
                    seen.push_back(new);
                }
            }
        }
        if point.1 > 0 {
            let new = (point.0, point.1 - 1);
            if (distances[new.0][new.1] == 0 && new != start) || steps + 1 < distances[new.0][new.1] {
                if grid[new.0][new.1] <= grid[point.0][point.1] + 1 {
                    distances[new.0][new.1] = steps + 1;
                    seen.push_back(new);
                }
            }
        }
    }

    distances[end.0][end.1]
}

pub fn part_two(contents: &str) -> usize {
    let map: Vec<Vec<char>> = contents.split('\n').map(|s| s.chars().collect()).collect();
    let height: usize = map.len();
    let width: usize = map[0].len();

    let mut end = (0, 0);

    let mut grid: Vec<Vec<usize>> = vec![vec![0; width]; height];

    let mut locations_of_a: Vec<(usize, usize)> = vec![];

    for i in 0..height {
        for j in 0..width {
            if map[i][j] == 'S' {
                grid[i][j] = 0;
                locations_of_a.push((i, j));
            } else if map[i][j] == 'E' {
                end = (i, j);
                grid[i][j] = 26;
            } else {
                grid[i][j] = map[i][j] as usize - 'a' as usize;
                if map[i][j] == 'a' {
                    locations_of_a.push((i, j));
                }
            }
        }
    }

    let mut distances: Vec<Vec<usize>> = vec![vec![0; width]; height];
    distances[end.0][end.1] = 0;
    let mut seen: VecDeque<(usize, usize)> = VecDeque::new();
    seen.push_back(end);

    while let Some(point) = seen.pop_front() {
        let steps = distances[point.0][point.1];

        if point.0 + 1 < height {
            let new = (point.0 + 1, point.1);
            if (distances[new.0][new.1] == 0 && new != end) || steps + 1 < distances[new.0][new.1] {
                if grid[point.0][point.1] <= grid[new.0][new.1] + 1 {
                    distances[new.0][new.1] = steps + 1;
                    seen.push_back(new);
                }
            }
        }
        if point.0 > 0 {
            let new = (point.0 - 1, point.1);
            if (distances[new.0][new.1] == 0 && new != end) || steps + 1 < distances[new.0][new.1] {
                if grid[point.0][point.1] <= grid[new.0][new.1] + 1 {
                    distances[new.0][new.1] = steps + 1;
                    seen.push_back(new);
                }
            }
        }
        if point.1 + 1 < width {
            let new = (point.0, point.1 + 1);
            if (distances[new.0][new.1] == 0 && new != end) || steps + 1 < distances[new.0][new.1] {
                if grid[point.0][point.1] <= grid[new.0][new.1] + 1 {
                    distances[new.0][new.1] = steps + 1;
                    seen.push_back(new);
                }
            }
        }
        if point.1 > 0 {
            let new = (point.0, point.1 - 1);
            if (distances[new.0][new.1] == 0 && new != end) || steps + 1 < distances[new.0][new.1] {
                if grid[point.0][point.1] <= grid[new.0][new.1] + 1 {
                    distances[new.0][new.1] = steps + 1;
                    seen.push_back(new);
                }
            }
        }
    }

    let mut min = usize::MAX;

    for point in locations_of_a {
        if distances[point.0][point.1] != 0 {
            min = min.min(distances[point.0][point.1]);
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part_one_works() {
        let expected = 31;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = 29;
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
