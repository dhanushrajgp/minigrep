use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let content =
        fs::read_to_string(config.file_path).expect("should have been able to read the file");
    println!("Content:\n {}", content);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
