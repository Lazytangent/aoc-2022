use dotenvy::dotenv;

use cli::cli;

#[tokio::main]
async fn main() {
    dotenv().ok();

    cli::run().await;
}
