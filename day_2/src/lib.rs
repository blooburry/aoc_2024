pub fn validate_report(report: &str) -> usize {
    report
        .lines()
        .enumerate()
        .filter(|(_, l)| {
            let s = l.split(' ').map(|e| e.parse::<i16>().unwrap());
            let s = s.clone().zip(s.skip(1));

            s.clone().all(
                |e| safe_decrement(e.0, e.1),
            )
            || s.clone().all(
                |e| safe_decrement(e.1, e.0), // check if safe increment
            )
        })
        .inspect(|(i, _)| println!("report {} is safe", i + 1))
        .count()
}

fn safe_decrement(e1: i16, e2: i16) -> bool {
    let safe = (e1 - e2 >= 1) && (e1 - e2 <= 3);
    // println!("{} and {} are{} safe", e1, e2, if safe { "" } else { " not" });
    safe
}