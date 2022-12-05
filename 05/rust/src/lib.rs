use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let halves: Vec<String> = contents.split("\n\n").map(String::from).collect();
    let drawing: Vec<String> = halves[0].split('\n').map(String::from).collect();
    let moves: Vec<String> = halves[1].split('\n').map(String::from).collect();

    let mut count = 1;
    let mut spaces = 1;
    let stack_row = &drawing[drawing.len() - 1];

    while spaces + 4 < stack_row.len() {
        spaces += 4;
        count += 1;
    }

    let mut stacks: Vec<Vec<char>> = vec![vec![]; count];

    for i in (0..drawing.len() - 1).rev() {
        let row = drawing[i].as_bytes();
        let mut idx = 1;
        let mut stack = 0;

        while idx <= spaces {
            if row.len() > idx {
                if row[idx] != ' ' as u8 {
                    stacks[stack].push(row[idx] as char);
                }
            }

            idx += 4;
            stack += 1;
        }
    }

    println!("Stacks: {stacks:#?}");
}
