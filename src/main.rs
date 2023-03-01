use std::{env, process::exit};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        println!("An error ocurred on reading the file: {error}");
        exit(1)
    };
}