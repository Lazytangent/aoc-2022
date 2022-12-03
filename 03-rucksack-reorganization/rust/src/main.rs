use utils::{self, cli};

use rucksack::solve;

fn main() {
    let cli = cli::run();

    solve(cli.r#type);
}
