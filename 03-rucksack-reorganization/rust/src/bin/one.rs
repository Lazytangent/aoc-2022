use utils::{self, cli};

use rucksack::solve_one;

fn main() {
    let cli = cli::run();

    solve_one(cli.r#type);
}
