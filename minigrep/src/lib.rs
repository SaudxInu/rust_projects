use std::env;
use std::fs;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filepath: String,
    pub case_sensitive: bool,
}


impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filepath."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Self { query, filepath, case_sensitive })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filepath.clone())?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    }
    else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}


pub fn search_case_sensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn case_sensitive() {
        let query: &str = "duct";
        let contents: &str = "Rust.\nFast, Safe, Productive.\nPick Three.\nDuct tape.";

        assert_eq!(vec!["Fast, Safe, Productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents: &str = "Rust.\nFast, Safe, Productive.\nPick Three.\nTrust me.";

        assert_eq!(vec!["Rust.", "Trust me."], search_case_insensitive(query, contents));
    }
}
