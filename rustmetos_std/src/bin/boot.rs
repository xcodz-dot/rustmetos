use clap::{arg, App};
use configparser::ini::Ini;
use rustmetos_api::api::path;

static VERSION: &str = "v1.0.0";

fn main() {
    let app = App::new("boot")
        .about("Utility to set boot options in `init.conf`.")
        .version(VERSION)
        .subcommand(
            App::new("set")
            .about("Set boot options in the configuration.")
            .arg(
                arg!(
                    <executable> "Name of the executable to set as boot. (Must be inside the /bin directory)"
                )
            )
        );
    let args = app.get_matches();

    match args.subcommand() {
        Some(("set", sub_args)) => {
            let command_name = String::from(sub_args.value_of("executable").unwrap());
            let mut configuration = Ini::new();
            configuration
                .load(&path("/conf/init.conf"))
                .unwrap_or_default();

            configuration.remove_section("exec");
            configuration.set("exec", "name", Some(command_name.clone()));

            configuration
                .write(&path("/conf/init.conf"))
                .expect("Unable to write the modified configuration");
        }
        _ => (),
    }
}
