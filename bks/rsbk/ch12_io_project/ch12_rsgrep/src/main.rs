use std::env;
use std::fs;

fn main() {
    println!("rust grep");

    // cmd args
    let args: Vec<String> = env::args().collect();

    // parse args
    let query = &args[1];
    let file_path = &args[2];

    // print for debug
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // read file
    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");
    println!("fice cont: {}", contents);
}
