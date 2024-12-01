use std::collections::HashMap;

pub fn parse_locations(s: &str) -> (Vec<u32>, Vec<u32>) {

    let s = s
        .lines()
        .map(|l| l
            .split(' ')
            .filter(|e| *e != "")
            .map(|n| n.parse::<u32>().expect(&format!("cannot parse number: `{}`", n)))
        )
        .flatten()
        .enumerate()
        .partition::<Vec<_>, _>(|(i, _n)| i % 2 == 0);

    (s.0.into_iter().map(|e| e.1).collect(), s.1.into_iter().map(|e| e.1).collect())
}

pub fn count_frequencies(numbers: &Vec<u32>) -> HashMap<u32, usize> {
    numbers.iter()
        .fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        })
}