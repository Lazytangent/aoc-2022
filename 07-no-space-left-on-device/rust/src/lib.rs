use std::{collections::HashMap, str};

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

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
        } else {
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
    }

    let mut small_dirs: HashMap<String, u32> = HashMap::new();

    for (dir, size) in dir_sizes {
        if size < 100_000 {
            small_dirs.insert(dir, size);
        }
    }

    let sum_small_dirs: u32 = small_dirs.values().sum();
    println!("Part one solution: {sum_small_dirs}");
}
