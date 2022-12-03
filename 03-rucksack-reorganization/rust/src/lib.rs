use utils::fs::DataType;

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let rucksacks: Vec<String> = contents
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

pub fn solve_one(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let rucksacks: Vec<String> = contents
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
