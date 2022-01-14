use configparser::ini::Ini;
use rustmetos_api::api::path;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() >= 2 {
        if args[1] == "set" {
            if args.len() >= 3 {
                let command_name = &args[2];
                let mut configuration = Ini::new();
                configuration
                    .load(&path("/conf/init.conf"))
                    .unwrap_or_default();

                configuration.remove_section("exec");
                configuration.set("exec", "name", Some(command_name.clone()));

                configuration
                    .write(&path("/conf/init.conf"))
                    .expect("Unable to write the modified configuration");
            } else {
                println!("{{binary_name}} required")
            }
        } else if args.get(1).unwrap_or(&String::from("help")) == &"help" {
            println!(
                "\
Usage: boot [set] [args...]

Commands:
  set {{binary_name}}      Set boot binary
  help                   Show this message"
            )
        }
    }
}
