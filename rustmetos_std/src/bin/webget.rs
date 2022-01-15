use reqwest::blocking;
use std::env::args;
use std::fs;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!(
            "\
Usage:
    webget {{uri}} {{output_file}}"
        );
        return;
    }

    let url = &args[1];
    let dest = &args[2];

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
