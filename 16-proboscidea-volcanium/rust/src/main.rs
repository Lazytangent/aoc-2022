use utils;

use sixteenth::solve;

fn main() {
    let cli = utils::cli::run();

    let r#type = utils::cli::get_type(&cli);
    solve(r#type, cli.part);
}