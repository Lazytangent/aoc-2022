use utils;

use sixth::solve;

fn main() {
    let cli = utils::cli::run();

    solve(cli.r#type);
}
