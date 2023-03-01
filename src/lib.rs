use std::{fs, error::Error, vec};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    for result_line in search(&config.query, &file_content){
        println!("{result_line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let lower_query = query.to_lowercase();
    let mut line_matches: Vec<&str> = vec![];

    content.lines().for_each(|line| {
        let lower_line = line.to_lowercase();
        if lower_line.contains(&lower_query) {
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
    
    #[test]
    fn multiple_lines_case_insensitive(){
        let query = "prog";
        let content = "It's not only writers who can benefit from this free online tool.\nIf you're a PROgrammer who's working on a project where blocks of text are needed, this tool can be a great way to get that.\nIt's a good way to test your PROgramming and that the tool being created is working well.";
    
        assert_eq!(vec!["If you're a PROgrammer who's working on a project where blocks of text are needed, this tool can be a great way to get that.", "It's a good way to test your PROgramming and that the tool being created is working well."], search(query, content));
    }
}