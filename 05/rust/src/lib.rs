use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let halves: Vec<String> = contents.split("\n\n").map(String::from).collect();
    let drawing: Vec<String> = halves[0].split('\n').map(String::from).collect();
    let moves: Vec<String> = halves[1].split('\n').map(String::from).collect();

    println!("{drawing:#?}");
    println!("{moves:#?}");
}
