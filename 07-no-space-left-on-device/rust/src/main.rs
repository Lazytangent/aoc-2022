use utils;

use seventh::solve;

fn main() {
    let cli = utils::cli::run();

    solve(cli.r#type);
}
