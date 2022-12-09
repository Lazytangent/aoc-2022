use utils;

use PACKAGE::solve;

fn main() {
    let cli = utils::cli::run();

    solve(cli.r#type, cli.part);
}
