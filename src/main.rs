use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args.get(1).expect("Search String Expected");
    let file_path = args.get(2).expect("File Path Expected");
    println!("Searching for {query} in file {file_path}");
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file.");
    println!("File text:\n{file_contents}")
    // TODO: check for errors
}
