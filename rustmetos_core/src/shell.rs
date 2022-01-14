use crate::{home, input};
use std::env::{consts::OS, current_dir, set_current_dir};
use std::fs;
use std::process::Command;

pub static VERSION: &str = "0.1.0";

pub fn run() {
    println!("Core Shell {}", VERSION);
    println!(
        "You are provided only a few basic commands for your use!
Type 'help' to list them."
    );

    let mut current_directory: String;

    'shellloop: loop {
        current_directory = current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap_or(String::from("~Unknown~"));
        let command = input(&format!("{}>", current_directory));
        let command: Vec<&str> = command.split_ascii_whitespace().collect();

        if command[0].trim() != "" {
            match command[0] {
                "help" => {
                    println!(
                        "A good list of basic commands:
    cd {{dir}}            Change working directory
    rm {{file}}           Remove a file
    rmdir {{dir}}         Remove a Directory (Recursivly if needed)
    exit                Exit the shell
    exec {{bin}} {{args}}   Execute a binary from /bin
    ls                  List current directory
    mkdir {{dir}}         Make a new directory
    mk {{file}}           Make an empty file
    reset               Reset the system to enter setup on next boot"
                    );
                }
                "cd" => {
                    if command.len() >= 2 {
                        let directory_to_switch = command[1..].join(" ");
                        set_current_dir(directory_to_switch).unwrap();
                    } else {
                        println!("Not enough arguments to 'cd'");
                    }
                }
                "rm" => {
                    if command.len() >= 2 {
                        let file_to_remove = command[1..].join(" ");
                        fs::remove_file(file_to_remove).unwrap();
                    } else {
                        println!("Not enough arguments to 'rm'");
                    }
                }
                "rmdir" => {
                    if command.len() >= 2 {
                        let dir_to_remove = command[1..].join(" ");
                        fs::remove_dir_all(dir_to_remove).unwrap();
                    } else {
                        println!("Not enough arguments to 'rmdir'");
                    }
                }
                "exit" => {
                    break 'shellloop;
                }
                "exec" => {
                    if command.len() >= 2 {
                        Command::new(
                            home().clone().to_owned()
                                + &format!("/bin/{}", command[1])
                                + if OS == "windows" { ".exe" } else { "" },
                        )
                        .args(command[1..].iter())
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                    } else {
                        println!("Not enough arguments to 'exec'");
                    }
                }
                "ls" => {
                    let paths = fs::read_dir("./").unwrap();

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
                        fs::create_dir_all(dir_to_make).unwrap();
                    } else {
                        println!("Not enough arguments to 'mkdir'");
                    }
                }
                "mk" => {
                    if command.len() >= 2 {
                        let file_to_make = command[1..].join(" ");
                        fs::write(file_to_make, "").unwrap();
                    } else {
                        println!("Not enough arguments to 'mk'");
                    }
                }
                "reset" => {
                    fs::remove_dir_all(home()).unwrap();
                    println!("Reset Complete! You will enter into configurator on next boot");
                    break 'shellloop;
                }
                _ => {
                    println!("Unknown Command, Use 'help' to list commands");
                }
            }
        } // else just continue
    }
}
