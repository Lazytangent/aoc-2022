use std::{env, process, fs};

use chrono::Datelike;
use reqwest::header::COOKIE;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Day of the data set to get, defaults to today's date, if in December
    #[arg(short, long)]
    day: Option<u32>,
    /// Year of the data set to get, defaults to current year, or most recent year
    #[arg(short, long)]
    year: Option<i32>,
}

pub async fn run(args: Args) {
    let client = reqwest::Client::new();

    let session_cookie = match env::var("SESSION_TOKEN") {
        Ok(v) => {
            format!("session={}", v.trim())
        }
        Err(e) => {
            eprintln!("Error getting session token: {e:#?}");
            process::exit(1);
        }
    };

    let now = chrono::offset::Local::now().date_naive();
    let mut day = now.day();
    let mut year = now.year();
    let month = now.month();

    if month != 12 {
        year -= 1;
    }

    if let Some(d) = args.day {
        day = d;
    }
    if let Some(y) = args.year {
        year = y;
    }

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let body = client.get(&url)
        .header(COOKIE, session_cookie)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    fs::create_dir_all("data").unwrap();

    fs::write("data/full.txt", body).unwrap();
}
