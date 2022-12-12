use utils;

use thirteenth::solve;

fn main() {
    let cli = utils::cli::run();

    solve(cli.r#type, cli.part);
}
