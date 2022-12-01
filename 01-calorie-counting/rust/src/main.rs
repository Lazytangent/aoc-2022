use utils::{self, fs::DataType};

fn main() {
    let contents = utils::fs::read_data(DataType::Real);

    let elves: Vec<String> = contents
        .split("\n\n")
        .map(String::from)
        .collect();

    let mut highest = 0;
    let (mut second, mut third) = (0, 0);

    for elf in elves {
        let calories: usize = elf
            .trim_end()
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

    println!("First solution: {highest:#?}");

    let top_three = highest + second + third;
    println!("Second solution: {top_three}");
}
