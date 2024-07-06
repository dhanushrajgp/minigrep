use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let query = &args[0];
    let file_path = &args[1];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    dbg!(args);
}
