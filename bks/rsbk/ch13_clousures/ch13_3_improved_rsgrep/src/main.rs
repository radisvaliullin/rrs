use ch13_3_improved_rsgrep::Config;
use std::env;
use std::process;

fn main() {
    // println!("rust grep");

    // parse config from args
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("rsgrep: problem parsing arguments: {err}");
        process::exit(1);
    });

    // print for debug
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // read file
    if let Err(e) = ch13_3_improved_rsgrep::run(config) {
        eprintln!("rsgrep error: {e}");
        process::exit(1);
    }
}
