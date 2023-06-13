use std::error::Error;
use std::{fs, env};
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => {arg},
            None => {return Err("Didn't get a query string")},
        };

        let file_path = match args.next() {
            Some(arg) => {arg},
            None => {return Err("Didn't get a file path")},
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}


/// Search string query from given string contents.
/// 
/// # Examples
/// ```
/// let result:Vec<&str> = search("pick", contents);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "Pick";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
pick two.";

        assert_eq!(vec!["Pick three."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "picK";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Pick three."], search_case_insensitive(query, contents));
    }
}