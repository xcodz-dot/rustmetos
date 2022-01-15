use clap::Parser;
use reqwest::blocking;
use std::fs;

#[derive(Parser)]
#[clap()]
struct Args {
    /// URI to download from
    uri: String,

    /// File to store the downloaded content
    destination: String,
}

fn main() {
    let args = Args::parse();

    let url = &args.uri;
    let dest = &args.destination;

    let mut response = blocking::get(url).unwrap();

    if !response.status().is_success() {
        println!(
            "Unsuccessful Request: {} {}",
            response.status().as_str(),
            response.status().canonical_reason().unwrap_or("")
        );
        return;
    } else {
        println!(
            "{} {}\nDownloading...",
            response.status().as_str(),
            response.status().canonical_reason().unwrap_or("")
        )
    }

    let mut file = fs::File::create(dest).unwrap();

    let bytes_wrote = response.copy_to(&mut file).unwrap();
    println!("Downloaded {} bytes", bytes_wrote);
}
