use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // parse args
        let query = args[1].clone();
        let file_path = args[2].clone();
        // result
        Ok(Config { query, file_path })
    }
}

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
    if let Err(e) = run(config) {
        println!("rsgrep error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("file content:\n{contents}");

    Ok(())
}
