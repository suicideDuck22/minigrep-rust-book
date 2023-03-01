use std::{fs, error::Error, vec};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    for result_line in search(&config.query, &file_content){
        println!("{result_line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut line_matches: Vec<&str> = vec![];
    content.lines().for_each(|line| {
        if line.contains(query) {
            line_matches.push(line);
        }
    });
    
    line_matches
}

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_one_line(){
        let query = "duct";
        let content = "\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}