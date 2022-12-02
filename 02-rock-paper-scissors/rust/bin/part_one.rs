use utils::{self, fs::DataType};

fn main() {
    let contents = utils::fs::read_data(DataType::Real);

    let rounds: Vec<String> = contents
        .trim_end()
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

    println!("First solution: {score}");
}
