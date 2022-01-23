use ini::ini;
use rustmetos_core::{home, init, input};
use std::env::consts::EXE_SUFFIX;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    init();

    println!("Welcome to RustMetOS");

    let rustmetos_home_path = Path::new(home());
    if !rustmetos_home_path.exists() {
        // If the home directory does not exist
        // Create and configure home
        println!("Configuring Home");

        fs::create_dir_all(rustmetos_home_path).expect("Unable to create a home directory");

        // After creating the home directory, create root components
        fs::create_dir_all(home().clone().to_owned() + "/bin").unwrap();
        fs::create_dir_all(home().clone().to_owned() + "/conf").unwrap();
        fs::create_dir_all(home().clone().to_owned() + "/home").unwrap();

        // Create a root user
        let username = input("Name for a root user> ");
        fs::create_dir_all(format!("{}/home/{}", home(), username)).unwrap();
        fs::write(
            format!("{}/conf/users.conf", home()),
            format!("[user]\nname={}", username),
        )
        .expect("Unable to write user file");

        // Input a installation archive that contains core file structure and overwrite it on home
        let archive_path =
            input("Home has been established!\n\nProvide installation archive's path> ");
        let archive_file = fs::File::open(archive_path).unwrap();
        let mut archive = zip::ZipArchive::new(archive_file).unwrap();

        archive
            .extract(home().clone().to_owned())
            .expect("Archive extraction unsuccesful");
        println!("Archive extraction successful! Restart to begin!");
        return;
    }

    println!("Loading INIT Configuration");
    let conf_file_exists = Path::new(&(home().clone().to_owned() + "/conf/init.conf")).exists(); // Open the init.conf file
    let mut executable_name = String::from("internal:core_shell");
    if !conf_file_exists {
        println!("Init configuration not found defaulting to core shell");
    } else {
        let init_conf = ini!(&(home().clone().to_owned() + "/conf/init.conf"));
        executable_name = init_conf["exec"]["name"]
            .clone()
            .unwrap_or(String::from("internal:core_shell"));
    }

    if executable_name == "internal:core_shell" {
        rustmetos_core::shell::run();
    } else {
        #[cfg(unix)]
        {
            Command::new("chmod")
                .arg("+x")
                .arg(home().clone().to_owned() + "/bin/" + &executable_name)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }

        Command::new(home().clone().to_owned() + "/bin/" + &executable_name + EXE_SUFFIX)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
