use std::{
    cmp::Ordering::{Greater, Less},
    collections::VecDeque,
};

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType, part: u8) {
    let contents = utils::fs::read_data(r#type);

    let val = match part {
        1 => part_one(&contents),
        2 => part_two(&contents),
        _ => unreachable!(),
    };

    println!("Part {part} solution: {val:?}");
}

pub fn part_one(contents: &str) -> usize {
    let packets: Vec<&str> = contents.lines().filter(|line| line.len() != 0).collect();

    packets
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, w)| compare(w[0], w[1]) == Less)
        .map(|(i, _)| i + 1)
        .sum()
}

fn compare(left: &str, right: &str) -> std::cmp::Ordering {
    let mut left = scanner(left);
    let mut right = scanner(right);

    loop {
        let ltoken = left.pop_front();
        let rtoken = right.pop_front();

        if ltoken.is_none() {
            return Less;
        }
        if rtoken.is_none() {
            return Greater;
        }

        let (ltoken, rtoken) = (ltoken.unwrap(), rtoken.unwrap());
        match (ltoken, rtoken) {
            (Token::Open, Token::Open) => (),
            (Token::Close, Token::Close) => (),
            (Token::Open, Token::Number(_)) => {
                right.push_front(Token::Close);
                right.push_front(rtoken.clone());
            }
            (Token::Number(_), Token::Open) => {
                left.push_front(Token::Close);
                left.push_front(ltoken.clone());
            }
            (Token::Number(l), Token::Number(r)) => {
                if l > r {
                    return Greater;
                } else if l < r {
                    return Less;
                }
            }
            (Token::Close, _) => return Less,
            (_, Token::Close) => return Greater,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Token {
    Open,
    Close,
    Number(usize),
}

fn scanner(s: &str) -> VecDeque<Token> {
    let mut tokens = VecDeque::<Token>::new();
    let mut temp = String::new();

    for c in s.chars() {
        match c {
            '[' => tokens.push_back(Token::Open),
            ']' => {
                if temp.len() > 0 {
                    tokens.push_back(Token::Number(temp.parse::<usize>().unwrap()));
                    temp.clear();
                }
                tokens.push_back(Token::Close);
            }
            ',' => {
                if temp.len() > 0 {
                    tokens.push_back(Token::Number(temp.parse::<usize>().unwrap()));
                    temp.clear();
                }
            }
            _ => temp.push(c),
        };
    }

    tokens
}

pub fn part_two(contents: &str) -> usize {
    let mut packets: Vec<&str> = contents.lines().filter(|line| line.len() != 0).collect();

    const MARKER_1: &str = "[[2]]";
    const MARKER_2: &str = "[[6]]";
    packets.push(MARKER_1);
    packets.push(MARKER_2);

    packets.sort_by(|left, right| compare(left, right));
    let index1 = packets.iter().position(|x| *x == MARKER_1).unwrap() + 1;
    let index2 = packets.iter().position(|x| *x == MARKER_2).unwrap() + 1;

    index1 * index2
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part_one_works() {
        let expected = 13;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = 140;
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
