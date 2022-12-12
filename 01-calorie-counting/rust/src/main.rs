use utils::{self, fs::DataType};

fn main() {
    let contents = utils::fs::read_data(DataType::Real);
    let highest = part_one(&contents);
    let top_three = part_two(&contents);

    println!("First solution: {highest:#?}");
    println!("Second solution: {top_three}");
}

fn part_one(contents: &str) -> usize {

    let elves: Vec<String> = contents
        .split("\n\n")
        .map(String::from)
        .collect();

    let mut highest = 0;
    let (mut second, mut third) = (0, 0);

    for elf in elves {
        let calories: usize = elf
            .split('\n')
            .map(|s| str::parse(s).unwrap())
            .reduce(|accum, item| accum + item)
            .unwrap();

        if calories >= highest {
            third = second;
            second = highest;
            highest = calories;
        } else if calories >= second {
            third = second;
            second = calories;
        } else if calories > third {
            third = calories;
        }
    }

    highest
}

fn part_two(contents: &str) -> usize {
    let elves: Vec<String> = contents
        .split("\n\n")
        .map(String::from)
        .collect();

    let mut highest = 0;
    let (mut second, mut third) = (0, 0);

    for elf in elves {
        let calories: usize = elf
            .split('\n')
            .map(|s| str::parse(s).unwrap())
            .reduce(|accum, item| accum + item)
            .unwrap();

        if calories >= highest {
            third = second;
            second = highest;
            highest = calories;
        } else if calories >= second {
            third = second;
            second = calories;
        } else if calories > third {
            third = calories;
        }
    }

    highest + second + third
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_one_works() {
        let expected = 24000;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = 45000;
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
