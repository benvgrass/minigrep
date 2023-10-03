use std::error::Error;
use std::fs;

pub struct QueryConfig<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
    pub case_insensitive: bool
}

impl<'a> QueryConfig<'a> {
    pub fn build(args: &'a[String]) -> Result<QueryConfig, &'static str> {
        if args.len() < 3 {
            Err("query string and file path expected")
        } else {
            Ok(QueryConfig {query: &args[1], file_path: &args[2]})
        }
    }

}

pub fn run(config: QueryConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_insensitive {
        search_case_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut search_results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            search_results.push(line);
        }
    }
    search_results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut search_results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            search_results.push(line);
        }
    }
    search_results
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