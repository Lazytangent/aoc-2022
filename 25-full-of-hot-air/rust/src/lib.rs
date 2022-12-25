use utils::{self, cli::Part, fs::DataType};

pub fn solve(r#type: DataType, part: Part) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        Part::One => part_one(&contents),
        Part::Two => part_two(&contents),
    };

    println!("Part {part} solution: {val:?}");
}

pub fn part_one(contents: &str) -> String {
    let fuel = contents.lines().map(snafu_to_int).sum();
    int_to_snafu(fuel)
}

fn snafu_to_int(snafu: &str) -> i64 {
    snafu.chars().fold(0i64, |res, ch| {
        res * 5
            + match ch {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            }
    })
}

fn int_to_snafu(mut n: i64) -> String {
    let mut snafu = String::new();

    while n != 0 {
        let rem = n.rem_euclid(5);
        n = n.div_euclid(5) + i64::from(rem > 2);
        snafu.insert(0, match rem {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        });
    }

    snafu
}

pub fn part_two(contents: &str) -> String {
    unreachable!()
}
