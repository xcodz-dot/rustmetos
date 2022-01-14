use std::io;
use std::io::Write;

pub mod shell;
static mut HOME: String = String::new();

pub fn init() {
    let homedir = home::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    unsafe {
        HOME = String::from(homedir + "/.rustmetos");
    }
}

pub fn home() -> &'static str {
    unsafe { &HOME }
}

pub fn input(prompt: &str) -> String {
    print!("{}", prompt); // Print the prompt without newline
    io::stdout()
        .flush()
        .expect("Unable to flush the stdout stream");

    let mut buffer = String::new(); // Create a input buffer

    // Read from stdin into the buffer
    io::stdin()
        .read_line(&mut buffer)
        .expect("Unable to read into the buffer");

    // Trim the newline off the buffer
    trim_newline(&mut buffer);

    buffer // Return the buffer as the input
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
