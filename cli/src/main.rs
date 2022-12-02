use dotenvy::dotenv;

use aoc_cli::cli;

#[tokio::main]
async fn main() {
    dotenv().ok();

    cli::run().await;
}
