use utils;

use eighth::solve;

fn main() {
    let cli = utils::cli::run();

    solve(cli.r#type);
}
