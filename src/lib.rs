use std::{fs, error::Error, vec, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search_case_sensitive(&config.query, &file_content)
    };

    for line_result in results{
        println!("{line_result}");
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut line_matches: Vec<&str> = vec![];

    content.lines().for_each(|line| {
        if line.contains(&query) {
            line_matches.push(line);
        }
    });
    
    line_matches
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut line_matches: Vec<&str> = vec![];

    content.lines().for_each(|line| {
        if line.to_lowercase().contains(&query) {
            line_matches.push(line);
        }
    });
    
    line_matches
}

struct ArgsTypes {
    flags: Vec<String>,
    params: Vec<String>
}

fn arguments_parser(args: &Vec<String>) -> Result<ArgsTypes, String> {
    if args.len() < 3 {
        return Err(format!("Is expected 2 arguments, but received {} arguments.", args.len() - 1));
    }

    if args.len() == 3 {
        return Ok(ArgsTypes {
            flags: vec![],
            params: vec![args[1].clone(), args[2].clone()]
        });
    }

    let slice_size_flags_args = args.len() - 2;
    let flags_args: &[String] = &args[1..slice_size_flags_args];
    let params_args: &[String] = &args[slice_size_flags_args..];
    
    Ok(ArgsTypes { flags: flags_args.to_vec(), params: params_args.to_vec() })
    
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, String> {
        let args: ArgsTypes = arguments_parser(&args)?;
    
        let query: String = args.params[0].clone();
        let file_path: String = args.params[1].clone();
        let ignore_case: bool;

        if env::var("IGNORE_CASE").is_ok() || (!args.flags.is_empty() && args.flags.contains(&String::from("--ignore-case"))) {
            ignore_case = true;
        } else {
            ignore_case = false;
        }

        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let content = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, content));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUst";
        let content = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    } 
}