use utils;

use twentyfifth::solve;

fn main() {
    let cli = utils::cli::run();

    let r#type = utils::cli::get_type(&cli);
    let part = utils::cli::get_part(&cli);
    solve(r#type, part);
}
