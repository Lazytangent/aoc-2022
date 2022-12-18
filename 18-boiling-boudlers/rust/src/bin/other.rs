use std::str::FromStr;
use ndarray::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() ->  &'static str {
"2,2,2
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
2,3,5"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input()), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input()), 58);
    }

}

struct Grid {
    // 0 = air, 1 = lava, 2 = vacuum
    grid: Array3<u8>
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Array3::zeros((24,24,24));
        s.lines().map(|l|
            l.trim().split(',').collect::<Vec<_>>()
        ).for_each(|v| {
            let x = v[0].parse::<usize>().unwrap();
            let y = v[1].parse::<usize>().unwrap();
            let z = v[2].parse::<usize>().unwrap();
            grid[[x+1,y+1,z+1]] = 1;
        });
        Ok(Grid { grid })
    }
}

impl Grid {
    fn sides(&self) -> i64 {
        let mut sides = 0;
        for x in 1..23 {
            for y in 1..23 {
                for z in 1..23 {
                    if self.grid[[x,y,z]] == 1 {
                        if self.grid[[x+1,y,z]] == 0 { sides += 1; }
                        if self.grid[[x-1,y,z]] == 0 { sides += 1; }
                        if self.grid[[x,y+1,z]] == 0 { sides += 1; }
                        if self.grid[[x,y-1,z]] == 0 { sides += 1; }
                        if self.grid[[x,y,z+1]] == 0 { sides += 1; }
                        if self.grid[[x,y,z-1]] == 0 { sides += 1; }
                    }
                }
            }
        }
        sides
    }

    fn set_vacuum(&mut self) {
        for x in 1..23 {
            for y in 1..23 {
                for z in 1..23 {
                    if self.grid[[x,y,z]] == 0 {
                        self.grid[[x,y,z]] = 2;
                    }
                }
            }
        }
    }

    fn air_expansion(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for x in 1..23 {
                for y in 1..23 {
                    for z in 1..23 {
                        if self.grid[[x,y,z]] == 2 &&
                            ( self.grid[[x+1,y,z]] == 0 ||
                                self.grid[[x-1,y,z]] == 0 ||
                                self.grid[[x,y+1,z]] == 0 ||
                                self.grid[[x,y-1,z]] == 0 ||
                                self.grid[[x,y,z+1]] == 0 ||
                                self.grid[[x,y,z-1]] == 0 ){
                            changed = true;
                            self.grid[[x,y,z]] = 0;
                         }
                    }
                }
            }
        }
    }
}


fn part1(input: &str) -> i64 {
    let grid: Grid = input.parse().unwrap();
    grid.sides()
}


fn part2(input: &str) -> i64 {
    let mut grid: Grid = input.parse().unwrap();
    grid.set_vacuum();
    grid.air_expansion();
    grid.sides()
}

fn main() {
    let input = include_str!("../../../data/full.txt");
    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {}us", time.elapsed().as_micros());
    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {}us", time.elapsed().as_micros());
}
