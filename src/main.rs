use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {err}");
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("in file '{}'", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Error reading file!");

    println!("with text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Expected 2 (search term & file).");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
