use utils::{self, fs::DataType};

fn main() {
    let contents = utils::fs::read_data(DataType::Real);

    let rucksacks: Vec<String> = contents
        .trim_end()
        .split('\n')
        .map(String::from)
        .collect();

    let mut sum = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        for c in rucksacks[i].chars() {
            if rucksacks[i+1].contains(c) && rucksacks[i+2].contains(c) {
                let priority = if c.is_lowercase() {
                    (c as u8 - 'a' as u8 + 1) as usize
                } else {
                    (c as u8 - 'A' as u8) as usize + 27
                };

                sum += priority;
                break;
            }
        }
    }

    println!("Second solution: {sum}");
}
