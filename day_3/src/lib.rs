use regex::Regex;

pub fn find_instructions(s: &str) -> Vec<InstructionMul> {
    
    let instruction_re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don\'t\(\)").unwrap();
    let arg_re = Regex::new(r"\d+").unwrap();
    let mut en = true;

    instruction_re.find_iter(s)
        .map(|c|{
            let c_start = c.as_str().chars().take(3).collect::<String>();
            // println!("found instruction: {}", c.as_str());
            return match c_start.as_str() {
                "do(" => { en = true; None},
                "don" => { en = false; None},
                &_ => {
                    // println!("instructions are {}", if en { "enabled" } else { "disabled" });
                    if !en { return None };
                    let mut args = arg_re.find_iter(c.as_str())
                    .map(|m|
                        m.as_str().parse::<i32>().expect(&format!("Cannot parse match: {}", m.as_str()))
                    );
                    let arg0 = args.next().unwrap();
                    let arg1 = args.next().unwrap();
                    // println!("adding instruction mul({}, {})", arg0, arg1);
                    Some(InstructionMul {
                        arg0,
                        arg1,
                    })
                }
            }
        })
        .filter(|e| e.is_some()) 
        .map(|e| e.unwrap()) 
        .collect::<Vec<_>>()
}

pub fn execute_instruction(ins: &InstructionMul) -> i32 {
    ins.arg0 * ins.arg1
}

#[derive(Debug)]
pub struct InstructionMul {
    arg0: i32,
    arg1: i32,
}
