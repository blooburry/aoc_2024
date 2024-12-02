use std::{env, fs, io};
use day_2::{validate_report};


fn main() {
    let input= get_input().expect("to read the input");
    println!("Amount of safe reports: {}", validate_report(&input));
}

fn get_input() -> Result<String, io::Error> { // $ cargo run -- resources/test0
    let args:Vec<String> = env::args().collect();
    let file_path = env::current_dir()?
        .join(args.get(1).expect("file path should be given"))
        .canonicalize().expect("cannot find file");
    fs::read_to_string(file_path)
}