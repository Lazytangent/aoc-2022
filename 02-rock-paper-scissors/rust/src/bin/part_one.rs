use utils::{self, fs::DataType};

fn main() {
    let contents = utils::fs::read_data(DataType::Real);
    let score = part_one(&contents);

    println!("First solution: {score}");
}

fn part_one(contents: &str) -> i32 {
    let rounds: Vec<String> = contents
        .split('\n')
        .map(String::from)
        .collect();

    let mut score = 0;

    for round in rounds {
        let strategy: Vec<&str> = round.split_whitespace().collect();

        match strategy[1] {
            "X" => {
                score += 1;

                match strategy[0] {
                    "A" => score += 3,
                    "C" => score += 6,
                    _ => ()
                };
            }
            "Y" => {
                score += 2;

                match strategy[0] {
                    "A" => score += 6,
                    "B" => score += 3,
                    _ => ()
                };
            }
            "Z" => {
                score += 3;

                match strategy[0] {
                    "B" => score += 6,
                    "C" => score += 3,
                    _ => ()
                };
            }
            _ => ()
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::part_one;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part_one_works() {
        let expected = 15;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }
}
