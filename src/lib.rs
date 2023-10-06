use std::error::Error;
use std::{fs, env};

pub struct QueryConfig {
    pub query: String,
    pub file_path: String,
    pub case_insensitive: bool
}

impl QueryConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<QueryConfig, &'static str> {
        args.next();
        let query = match args.next() {
            Some(q) => q,
            None => return Err("query not provided")
        };
        let file_path = match args.next() {
            Some(fp) => fp,
            None => return Err("file path not provided")
        };
        let case_insensitive = env::var("IGNORE_CASE").is_ok();

        Ok(QueryConfig {query, file_path, case_insensitive})

    }

}

pub fn run(config: QueryConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(query)).collect()
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
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );

    }
}