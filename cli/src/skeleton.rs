use clap::Subcommand;

mod generate;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Command to run
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate the skeleton
    Generate(generate::Args),
}

pub async fn run(args: Args) {
    match args.command {
        Command::Generate(args) => generate::run(args).await,
    }
}
