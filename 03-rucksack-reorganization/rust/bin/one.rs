use utils::{self, fs::DataType};

fn main() {
    let contents = utils::fs::read_data(DataType::Real);

    let rucksacks: Vec<String> = contents
        .trim_end()
        .split('\n')
        .map(String::from)
        .collect();

    let mut sum = 0;

    for rucksack in rucksacks {
        let n = rucksack.len() / 2;
        for c in rucksack[..n].chars() {
            if let Some(_) = rucksack[n..].find(c) {
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

    println!("First solution: {sum}");
}
