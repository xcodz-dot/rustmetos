use clap::{app_from_crate, arg};
use std::fs;

static VERSION: &str = "v1.0.0";

fn main() {
    let app = app_from_crate!()
        .about("Read a text file and print it to console")
        .version(VERSION)
        .arg(arg!(
            <file> "File to read and display from."
        ));

    let args = app.get_matches();

    let content: String = fs::read_to_string(args.value_of("file").unwrap()).unwrap();
    println!("{}", content);
}
