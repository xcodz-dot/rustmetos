use std::env::args;
use std::fs::File;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() <= 2 {
        println!("Not enough arguments to unzip\nplease provide a file to extract and a directory to extract to.");
        return;
    }

    let archive_path = &args[1];
    let archive_file = File::open(archive_path).unwrap();
    let mut archive = zip::ZipArchive::new(archive_file).unwrap();

    archive
        .extract(&args[2])
        .expect("Archive extraction unsuccesful");
}
