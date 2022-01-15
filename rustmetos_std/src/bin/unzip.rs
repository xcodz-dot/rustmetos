use clap::{arg, App};
use std::fs::File;

static VERSION: &str = "v1.0.0";

fn main() {
    let app = App::new("unzip")
        .about("Extracts zip archives into a destination folder.")
        .version(VERSION)
        .arg(arg!(
            <zipfile> "Zip file to extract."
        ))
        .arg(arg!(
            <destination> "Destination directory to extract to."
        ));

    let args = app.get_matches();

    let archive_path = args.value_of("zipfile").expect("Zip file is required");
    let archive_file = File::open(archive_path).unwrap();
    let mut archive = zip::ZipArchive::new(archive_file).unwrap();

    archive
        .extract(
            args.value_of("destination")
                .expect("Destination is required"),
        )
        .expect("Archive extraction unsuccesful");
}
