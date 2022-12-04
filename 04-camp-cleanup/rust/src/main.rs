use utils::cli;

use cleanup::solve;

fn main() {
    let cli_ = cli::run();

    solve(cli_.r#type);
}
