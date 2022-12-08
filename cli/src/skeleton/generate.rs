use std::fs;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Name of the new package
    name: String,
}

pub async fn run(args: Args) {
    let client = reqwest::Client::new();

    let toml_file = get_toml_file(&client, &args.name).await;
    let main_file = get_main_file(&client, &args.name).await;
    let lib_file = get_lib_file(&client).await;

    create_file_structure();
    fs::write("rust/Cargo.toml", toml_file).unwrap();
    fs::write("rust/src/main.rs", main_file).unwrap();
    fs::write("rust/src/lib.rs", lib_file).unwrap();
}

async fn get_file(client: &reqwest::Client, name: &str) -> String {
    let url = format!("https://raw.githubusercontent.com/Lazytangent/aoc-2022/main/cli/skeletons/{}", name);

    client.get(&url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

async fn get_main_file(client: &reqwest::Client, name: &str) -> String {
    let file = get_file(client, "main.rs").await;

    file.replace("PACKAGE", name)
}

async fn get_lib_file(client: &reqwest::Client) -> String {
    get_file(client, "lib.rs").await
}

async fn get_toml_file(client: &reqwest::Client, name: &str) -> String {
    let file = get_file(client, "Cargo.toml").await;

    file.replace("PACKAGE", name)
}

fn create_file_structure() {
    fs::create_dir_all("rust/src").unwrap();
}
