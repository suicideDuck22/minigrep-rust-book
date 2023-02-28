use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        exit(1);
    });

    println!("{}, {}", config.query, config.file_path);
    let file_content = fs::read_to_string(config.file_path).expect("No file found");
    println!("What the text: \n {file_content}");
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Is expected 2 arguments, but received {} arguments.", args.len() - 1));
        }
    
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
    
        Ok(Config {
            query,
            file_path
        })
    }
}