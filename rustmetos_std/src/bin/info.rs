use clap::{arg, App, ArgGroup};
use rustmetos_api::api::path;
use std::fs;
use std::path::PathBuf;

static VERSION: &str = "v1.0.0";

fn main() {
    let app = App::new("info")
        .version(VERSION)
        .about("Info command is the local manual of the system.")
        .long_about(
            "It provides you with a way to navigate documentation
of commands provided by commands in markdown format.",
        )
        .arg(arg!(
            -l --list ... "List arguments"
        ))
        .arg(arg!(
            [topic] "Name of topic or command"
        ))
        .group(
            ArgGroup::new("")
                .arg("list")
                .arg("topic")
                .multiple(false)
                .required(true),
        );
    let args = app.get_matches();

    let topics = list_topics().expect("Unable to obtain list of topics");

    if args.is_present("list") {
        println!("{}", topics.join("\n"));
    } else if topics.contains(&String::from(args.value_of("topic").unwrap())) {
        let content = fs::read_to_string(path(&format!(
            "/data/info/{}.md",
            args.value_of("topic").unwrap()
        )))
        .expect("Unable to read the information from the info file");
        println!(
            "Info on topic {}:\n\n {}",
            args.value_of("topic").unwrap(),
            content
        );
    } else {
        println!(
            "The given topic does not exist, please type info list to list all available topics"
        );
    }
}

fn list_topics() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let directory_listing: Vec<PathBuf> = fs::read_dir(path("/data/info"))?
        .map(|path| path.unwrap().path())
        .collect();

    let mut filenames: Vec<String> = Vec::with_capacity(directory_listing.capacity());

    for x in directory_listing {
        match x.file_name() {
            Some(value) => {
                filenames.push(value.to_os_string().into_string().unwrap_or_default());
            }
            _ => (),
        }
    }

    for x in &mut filenames {
        // Remove ".md" extension from filenames
        *x = String::from(x.strip_suffix(".md").unwrap_or(x));
    }

    Ok(filenames)
}
