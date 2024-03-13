use std::env;
use std::process;
use ch12_rsgrep::Config;

fn main() {
    // println!("rust grep");

    // cmd args
    let args: Vec<String> = env::args().collect();

    // parse config from args
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("rsgrep: problem parsing arguments: {err}");
        process::exit(1);
    });

    // print for debug
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // read file
    if let Err(e) = ch12_rsgrep::run(config) {
        println!("rsgrep error: {e}");
        process::exit(1);
    }
}
