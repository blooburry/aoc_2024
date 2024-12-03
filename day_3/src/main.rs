use day_3::{execute_instruction, find_instructions};
use std::{env, fs, io};

fn main() {
    let input = get_input().expect("failed to get input");

    let ins = find_instructions(input.as_str());
    // println!("Instructions: {:?}", &ins);

    let res = ins.iter()
        .map(|i| execute_instruction(i))
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    println!("Sum of instruction results: {}", res);
}

fn get_input() -> Result<String, io::Error> { // $ cargo run -- resources/test0
    let args:Vec<String> = env::args().collect();
    let file_path = env::current_dir()?
        .join(args.get(1).expect("file path should be given"))
        .canonicalize().expect("cannot find file");
    fs::read_to_string(file_path)
}