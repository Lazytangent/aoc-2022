use utils;

use fifteenth::solve;

fn main() {
    let cli = utils::cli::run();

    solve(cli.r#type, cli.part);
}
