use std::{env, process};
use minigrep::QueryConfig;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = QueryConfig::build(&args).unwrap_or_else(|err| {
        println!("Issue parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Error running application: {e}");
        process::exit(1);
    }

}
