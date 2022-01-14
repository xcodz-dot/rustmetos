use std::env::args;
use std::fs;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() <= 1 {
        println!("A file name must be provided to read 'read {{filename}}'");
        return;
    }

    let content: String = fs::read_to_string(&args[1]).unwrap();
    println!("{}", content);
}
