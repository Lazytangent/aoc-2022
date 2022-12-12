use utils::{self, fs::DataType};

pub fn solve_one(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);
    let count = part_one(&contents);

    println!("Part one solution: {count}");
}

pub fn part_one(contents: &str) -> i32 {
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

    count
}

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);
    let count = part_two(&contents);

    println!("Part two solution: {count}");
}

pub fn part_two(contents: &str) -> i32 {
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

    count
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_one_works() {
        let expected = 2;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = 4;
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
