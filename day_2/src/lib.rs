pub fn validate_report(report: &str) -> usize {
    report
        .lines()
        .enumerate()
        .filter(|(i, l)| {
            let s = l
                .split(' ')
                .map(|e| e.parse::<i16>().unwrap())
                .collect::<Vec<_>>();
            
            let safe = check_report(&s, true) || check_report(&s, false);
            println!("report {} is {}", i + 1, if safe { "safe" } else { "unsafe" });
            safe
        })
        .count()
}

fn safe_step(e1: i16, e2: i16, descending: bool) -> bool {
    if descending {
        let safe = (e1 - e2 >= 1) && (e1 - e2 <= 3);
        println!("    {} desc. to {} is {}", e1, e2, if safe { "safe"} else { "unsafe" });
        safe
    } else {
        let safe = (e2 - e1 >= 1) && (e2 - e1 <= 3);
        println!("    {} asc. to {} is {}", e1, e2, if safe { "safe"} else { "unsafe" });
        safe
    }
}

fn check_report(s: &Vec<i16>, descending: bool) -> bool {
    let mut i = 0;
    let mut problem_dampener_used = false;

    while i <= s.len() - 2 {
        // don't need to check the very last one
        let e1 = *s.get(i).unwrap();
        let e2 = *s.get(i + 1).unwrap();
        if safe_step(e1, e2, descending) {
            i += 1;
            continue; // safe
        } else { 
            if problem_dampener_used { return false; }
            // unsafe

            // is e2 the problem number?
            let e3 = s.get(i + 2);
            if e3.is_none() { 
                return true;
            }
            println!("  {} and {} are unsafe, attempting to use problem dampener", e1, e2);
            if safe_step(e1, *e3.unwrap(), descending) {
                // use problem dampener
                println!("  problem dampener used on the second number");
                problem_dampener_used = true;
                i += 2; // skip the problem number
                continue;
            }

            // is e1 the problem number?
            if i == 0 { 
                problem_dampener_used = true;
                i += 1;
            } else if safe_step(*s.get(i - 1).unwrap(), e2, descending) { 
                // use problem dampener
                println!("  problem dampener used on the first number");
                problem_dampener_used = true;
                i += 1; 
                continue;
            } else {
                return false;
            }
        }
    }

    true
}
