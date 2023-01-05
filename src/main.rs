use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!(
                "Not enough arguments. Expected 2 but got {}",
                args.len() - 1
            );
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
