use utils;

use eleventh::solve;

fn main() {
    let cli = utils::cli::run();

    solve(cli.r#type, cli.part);
}
