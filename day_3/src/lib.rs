use regex::Regex;

pub fn find_instructions(s: &str) -> Vec<Instruction> {
    
    let instruction_re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let arg_re = Regex::new(r"\d+").unwrap();

    instruction_re.find_iter(s)
        .map(|c|{
            let mut args = arg_re.find_iter(c.as_str())
                .map(|m|
                    m.as_str().parse::<i32>().expect(&format!("Cannot parse match: {}", m.as_str()))
                );
            Instruction {
                arg0: args.next().unwrap(),
                arg1: args.next().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

pub fn execute_instruction(ins: &Instruction) -> i32 {
    ins.arg0 * ins.arg1
}

#[derive(Debug)]
pub struct Instruction {
    arg0: i32,
    arg1: i32,
}