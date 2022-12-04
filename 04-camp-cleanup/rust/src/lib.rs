use utils::{self, fs::DataType};

pub fn solve_one(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let pairs: Vec<String> = contents.split('\n').map(String::from).collect();

    let mut count = 0;

    for pair in pairs {
        let pair: Vec<&str> = pair.split(',').collect();
        let one: Vec<u32> = pair[0]
            .split('-')
            .map(|s| str::parse::<u32>(s).unwrap())
            .collect();
        let two: Vec<u32> = pair[1]
            .split('-')
            .map(|s| str::parse::<u32>(s).unwrap())
            .collect();

        if one[0] <= two[0] && one[1] >= two[1] {
            count += 1;
        } else if one[0] >= two[0] && one[1] <= two[1] {
            count += 1;
        }
    }

    println!("Part one solution: {count}");
}

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let pairs: Vec<String> = contents.split('\n').map(String::from).collect();

    let mut count = 0;

    for pair in pairs {
        let pair: Vec<&str> = pair.split(',').collect();
        let one: Vec<u32> = pair[0]
            .split('-')
            .map(|s| str::parse::<u32>(s).unwrap())
            .collect();
        let two: Vec<u32> = pair[1]
            .split('-')
            .map(|s| str::parse::<u32>(s).unwrap())
            .collect();

        if one[0] <= two[0] && one[1] >= two[0] {
            count += 1;
        } else if one[0] >= two[0] && one[0] <= two[1] {
            count += 1;
        } else if one[1] >= two[0] && one[1] <= two[1] {
            count += 1;
        } else if two[1] >= one[0] && two[1] <= one[1] {
            count += 1;
        }
    }

    println!("Part one solution: {count}");
}
