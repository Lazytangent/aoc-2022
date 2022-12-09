use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let GROUPS: Vec<String> = contents.split('\n').map(String::from).collect();
}
