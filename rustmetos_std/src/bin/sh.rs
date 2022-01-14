use rustmetos_api::api;
use rustmetos_core::input;
use std::env::{current_dir, set_current_dir};
use std::error::Error;
use std::fs;

pub static VERSION: &str = "1.0.0";

fn main() {
    println!("Rustmetos Shell (v{})", VERSION);
    println!("Type 'help' to list shell commands.");
    let mut current_directory: String;
    'shellloop: loop {
        current_directory = current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap_or(String::from("~Unknown~"));
        let command = input(&format!("{}>", current_directory));
        let command: Vec<&str> = command.split_ascii_whitespace().collect();
        if command.len() != 0 {
            if command[0].trim() != "" {
                if command[0] == "exit" {
                    break 'shellloop;
                } else {
                    match run_cmd(command) {
                        Err(err) => {
                            println!("Error: {}", err);
                        }
                        _ => {}
                    };
                }
            } // else just continue
        }
    }
}

fn run_cmd(command: Vec<&str>) -> Result<(), Box<dyn Error>> {
    match command[0] {
        "help" => {
            println!(
                "A good list of basic commands:
cd {{dir}}            Change working directory
rm {{file}}           Remove a file
rmdir {{dir}}         Remove a Directory (Recursivly if needed)
exit                Exit the shell
ls [dir]            List current directory
mkdir {{dir}}         Make a new directory
mk {{file}}           Make an empty file
home                Change Directory to home"
            );
        }
        "cd" => {
            if command.len() >= 2 {
                let directory_to_switch = command[1..].join(" ");
                set_current_dir(directory_to_switch)?;
            } else {
                println!("Not enough arguments to 'cd'");
            }
        }
        "home" => {
            set_current_dir(api::home())?;
        }
        "rm" => {
            if command.len() >= 2 {
                let file_to_remove = command[1..].join(" ");
                fs::remove_file(file_to_remove)?;
            } else {
                println!("Not enough arguments to 'rm'");
            }
        }
        "rmdir" => {
            if command.len() >= 2 {
                let dir_to_remove = command[1..].join(" ");
                fs::remove_dir_all(dir_to_remove)?;
            } else {
                println!("Not enough arguments to 'rmdir'");
            }
        }
        "ls" => {
            let mut paths = fs::read_dir("./")?;
            if command.len() == 2 {
                paths = fs::read_dir(command[1])?;
            }
            for path in paths {
                println!(
                    "{}: {}",
                    if path.as_ref().unwrap().path().is_dir() {
                        "d"
                    } else {
                        "f"
                    },
                    path.unwrap().path().display()
                );
            }
        }
        "mkdir" => {
            if command.len() >= 2 {
                let dir_to_make = command[1..].join(" ");
                fs::create_dir_all(dir_to_make)?;
            } else {
                println!("Not enough arguments to 'mkdir'");
            }
        }
        "mk" => {
            if command.len() >= 2 {
                let file_to_make = command[1..].join(" ");
                fs::write(file_to_make, "")?;
            } else {
                println!("Not enough arguments to 'mk'");
            }
        }
        _ => match api::exec(
            command[0],
            command[1..].iter().map(|string| *string).collect(),
        ) {
            Err(err) => {
                println!("Error Encountered: {}", err);
            }
            _ => {}
        },
    }
    Ok(())
}
