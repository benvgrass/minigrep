use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args.get(1).expect("Search String Expected");
    let file_path = args.get(2).expect("File Path Expected");
    println!("Searching for {query} in file {file_path}");
}
