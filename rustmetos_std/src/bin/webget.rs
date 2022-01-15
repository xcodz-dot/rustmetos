use clap::{arg, App};
use reqwest::blocking;
use std::fs;

static VERSION: &str = "v1.0.0";

fn main() {
    let app = App::new("webget")
        .about("Download from a uri and store it locally.")
        .version(VERSION)
        .arg(arg!(
            <uri> "URI to download from."
        ))
        .arg(arg!(
            <dest> "Destination to download to."
        ));
    let args = app.get_matches();

    let url = args.value_of("uri").unwrap();
    let dest = args.value_of("dest").unwrap();

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
