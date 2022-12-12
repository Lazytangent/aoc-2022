use utils::{self, fs::DataType};

fn main() {
    let contents = utils::fs::read_data(DataType::Real);
    let score = part_two(&contents);

    println!("Second solution: {score}");
}

fn part_two(contents: &str) -> i32 {
    let rounds: Vec<String> = contents.split('\n').map(String::from).collect();

    let mut score = 0;

    for round in rounds {
        let strategy: Vec<&str> = round.split_whitespace().collect();

        match strategy[1] {
            "X" => {
                match strategy[0] {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    _ => (),
                };
            }
            "Y" => {
                score += 3;

                match strategy[0] {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    _ => (),
                };
            }
            "Z" => {
                score += 6;

                match strategy[0] {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    _ => (),
                };
            }
            _ => (),
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::part_two;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part_two_works() {
        let expected = 12;
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
