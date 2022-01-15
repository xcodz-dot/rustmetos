use rustmetos_api::api::path;
use std::env::args;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!(
            "\
Usage:
    info {{name of topic}}
    info {{name of command}}
    info list"
        );
        return;
    }

    let topics = list_topics().expect("Unable to obtain list of topics");

    if &args[1] == "list" {
        println!("{}", topics.join("\n"));
    } else if topics.contains(&args[1]) {
        let content = fs::read_to_string(path(&format!("/data/info/{}.md", &args[1])))
            .expect("Unable to read the information from the info file");
        println!("Info on topic {}:\n\n {}", args[1], content);
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
