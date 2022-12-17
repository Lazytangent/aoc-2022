use utils::{self, cli::Part, fs::DataType};

pub fn solve(r#type: DataType, part: Part) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        Part::One => part_one(&contents),
        Part::Two => part_two(&contents),
    };

    println!("Part {part} solution: {val:?}");
}

#[derive(Debug, Clone, Copy)]
enum Wind {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Shape(u32);

impl Shape {
    const fn all_shapes() -> [Self; 5] {
        [
            Self(0x0000001E),
            Self(0x00081C08),
            Self(0x0004041C),
            Self(0x10101010),
            Self(0x00001818),
        ]
    }

    fn blow(&mut self, direction: Wind, mask: u32) {
        let new_pos = match direction {
            Wind::Left => {
                if self.0 & 0x40404040 == 0 {
                    self. 0 << 1
                } else {
                    return;
                }
            }
            Wind::Right => {
                if self.0 & 0x01010101 == 0 {
                    self.0 >> 1
                } else {
                    return;
                }
            }
        };

        if new_pos & mask == 0 {
            self.0 = new_pos;
        }
    }

    const fn intersects(&self, mask: u32) -> bool {
        self.0 & mask != 0
    }

    fn as_bytes(self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|b| *b != 0)
    }
}

fn tower_mask(tower: &[u8], height: usize) -> u32 {
    if height >= tower.len() {
        0
    } else {
        tower[height..]
            .iter()
            .take(4)
            .rev()
            .fold(0u32, |acc, b| (acc << 8) | *b as u32)
    }
}

fn drop_rock(tower: &mut Vec<u8>, wind: &[Wind], mut wind_idx: usize, mut shape: Shape) -> usize {
    let mut height = tower.len() + 3;

    loop {
        let wind_dir = wind[wind_idx];
        wind_idx += 1;
        if wind_idx == wind.len() {
            wind_idx = 0;
        }

        let current_mask = tower_mask(tower, height);

        shape.blow(wind_dir, current_mask);

        if height > tower.len() {
            height -= 1;
        } else if height == 0 || shape.intersects(tower_mask(tower, height - 1)) {
            for byte in shape.as_bytes() {
                if height < tower.len() {
                    tower[height] |= byte;
                } else {
                    tower.push(byte);
                }
                height += 1;
            }
            return wind_idx;
        } else {
            height -= 1;
        }
    }
}

pub fn part_one(contents: &str) -> usize {
    let num_rocks = 2022;
    let mut tower = Vec::with_capacity(num_rocks * 4);

    let input: Vec<Wind> = contents.chars().map(|c| {
        if c == '<' {
            Wind::Left
        } else {
            Wind::Right
        }
    }).collect();

    let mut wind_idx = 0;
    for shape in Shape::all_shapes().into_iter().cycle().take(num_rocks) {
        wind_idx = drop_rock(&mut tower, &input, wind_idx, shape);
    }

    tower.len()
}

pub fn part_two(contents: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::part_one;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 3068);
    }
}
