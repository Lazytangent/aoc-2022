use utils;

use ninth::solve;

fn main() {
    let cli = utils::cli::run();

    solve(cli.r#type);
}
