use std::{env, fs};
use day_1::{parse_locations, count_frequencies};

fn main() -> std::io::Result<()> {
    let p = env::current_dir()?;
    println!("Running in: {}", p.display());

    let args:Vec<String> = env::args().collect();
    let file_path = p
        .join(args.get(1).expect("file path should be given"))
        .canonicalize().expect("cannt find file");
    let contents = fs::read_to_string(file_path)?;

    let mut locations = parse_locations(&contents);

    println!("locations: {:#?}", locations);
    
    let map = count_frequencies(&locations.1);
    let similarity = locations.0.iter()
    .map(|e| 
        (map.get(e).unwrap_or(&0), e)
    )
    .fold(0, |acc, e| 
        acc + (*e.0 as usize) * (*e.1 as usize)
    );

    println!("similarity: {}", similarity);

    locations.0.sort();
    locations.1.sort();
    let sum_of_differences = locations.0.into_iter()
        .zip(locations.1.into_iter())
        .map(|e| e.0.abs_diff(e.1))
        .sum::<u32>();

    println!("sum of differences: {}", sum_of_differences);

    Ok(())
}
