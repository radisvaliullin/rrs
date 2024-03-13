use bks_rsbk_ch12_rg::Config;
use std::env;
use std::process;

fn main() {
    // println!("rust grep");

    // cmd args
    let args: Vec<String> = env::args().collect();

    // parse config from args
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("rsgrep: problem parsing arguments: {err}");
        process::exit(1);
    });

    // print for debug
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // read file
    if let Err(e) = bks_rsbk_ch12_rg::run(config) {
        eprintln!("rsgrep error: {e}");
        process::exit(1);
    }
}
