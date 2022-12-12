use std::{collections::HashMap, str};

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let part_one_solution = part_one(&contents);
    println!("Part one solution: {part_one_solution}");

    let part_two_solution = part_two(&contents);
    println!("Part two solution: {}", part_two_solution);
}

pub fn part_one(contents: &str) -> u32 {
    let mut parent_dirs: Vec<String> = vec![];
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    let lines: Vec<String> = contents.split('\n').map(String::from).collect();

    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();

        if line[0] == "$" {
            if line[1] == "cd" {
                if line[2] != ".." {
                    parent_dirs.push(line[2].to_string());
                    let path = parent_dirs.join("/");
                    dir_sizes.insert(path, 0);
                } else {
                    parent_dirs.pop();
                }
            }
            continue;
        }

        match str::parse::<u32>(line[0]) {
            Ok(v) => {
                let mut temp_parent_dirs = Vec::from(&parent_dirs[..]);
                while !temp_parent_dirs.is_empty() {
                    let path = temp_parent_dirs.join("/");
                    dir_sizes.entry(path).and_modify(|val| *val += v);
                    temp_parent_dirs.pop();
                }
            }
            Err(_) => (),
        }
    }

    let mut small_dirs: HashMap<String, u32> = HashMap::new();

    for (dir, size) in &dir_sizes {
        if size < &100_000 {
            small_dirs.insert(dir.clone(), *size);
        }
    }

    small_dirs.values().sum()
}

pub fn part_two(contents: &str) -> u32 {
    let mut parent_dirs: Vec<String> = vec![];
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    let lines: Vec<String> = contents.split('\n').map(String::from).collect();

    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();

        if line[0] == "$" {
            if line[1] == "cd" {
                if line[2] != ".." {
                    parent_dirs.push(line[2].to_string());
                    let path = parent_dirs.join("/");
                    dir_sizes.insert(path, 0);
                } else {
                    parent_dirs.pop();
                }
            }
            continue;
        }

        match str::parse::<u32>(line[0]) {
            Ok(v) => {
                let mut temp_parent_dirs = Vec::from(&parent_dirs[..]);
                while !temp_parent_dirs.is_empty() {
                    let path = temp_parent_dirs.join("/");
                    dir_sizes.entry(path).and_modify(|val| *val += v);
                    temp_parent_dirs.pop();
                }
            }
            Err(_) => (),
        }
    }

    let mut small_dirs: HashMap<String, u32> = HashMap::new();

    for (dir, size) in &dir_sizes {
        if size < &100_000 {
            small_dirs.insert(dir.clone(), *size);
        }
    }

    let total_disk_space: u32 = 70_000_000;
    let required_free_space: u32 = 30_000_000;

    let current_free_space = total_disk_space - dir_sizes["/"];
    let required_change = required_free_space - current_free_space;

    let mut eligible_dirs: HashMap<String, u32> = HashMap::new();

    for (dir, size) in &dir_sizes {
        if size >= &required_change {
            eligible_dirs.insert(dir.clone(), *size);
        }
    }

    let mut smallest_eligible_dir = "/".to_string();

    for (dir, size) in &eligible_dirs {
        if size < &eligible_dirs[&smallest_eligible_dir] {
            smallest_eligible_dir = dir.clone();
        }
    }

    eligible_dirs[&smallest_eligible_dir]
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part_one_works() {
        let expected = 95437;
        let actual = part_one(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_works() {
        let expected = 24933642;
        let actual = part_two(INPUT);

        assert_eq!(actual, expected);
    }
}
