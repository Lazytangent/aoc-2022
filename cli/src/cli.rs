use clap::{Parser, Subcommand};

use crate::{test, data, skeleton};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Command for testing things out
    Test,
    /// Get data set for a specific day
    Data(data::Args),
    /// For working with skeletons...
    Skeleton(skeleton::Args),
}

pub async fn run() {
    let cli = Cli::parse();

    match cli.command {
        Command::Test => test::run(),
        Command::Data(args) => data::run(args).await,
        Command::Skeleton(args) => skeleton::run(args).await,
    };
}
