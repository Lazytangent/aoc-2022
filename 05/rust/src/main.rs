use utils;

use fifth::solve;

fn main() {
    let cli = utils::cli::run();

    solve(cli.r#type);
}
