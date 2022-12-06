use utils;

use sixth::solve_one;

fn main() {
    let cli = utils::cli::run();

    solve_one(cli.r#type);
}
