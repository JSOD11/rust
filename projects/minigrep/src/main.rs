use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for: {:#?}", query);
    println!("File path: {:#?}", file_path);

    let file_contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    println!("File contents: {:#?}", file_contents);
}
