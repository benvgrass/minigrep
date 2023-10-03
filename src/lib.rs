use std::error::Error;
use std::fs;

pub struct QueryConfig<'a> {
    pub query: &'a String,
    pub file_path: &'a String
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

    println!("File Text:\n{contents}");
    Ok(())
}