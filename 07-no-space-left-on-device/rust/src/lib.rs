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

    for (dir, size) in &dir_sizes {
        if size < &100_000 {
            small_dirs.insert(dir.clone(), *size);
        }
    }

    let sum_small_dirs: u32 = small_dirs.values().sum();
    println!("Part one solution: {sum_small_dirs}");

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

    println!("Part two solution: {}", eligible_dirs[&smallest_eligible_dir]);
}
