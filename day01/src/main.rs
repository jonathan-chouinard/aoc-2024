use itertools::Itertools;
use std::{env, error::Error, fs};

fn load_input_file() -> String {
    let file_path = env::args().nth(1).expect("No input file passed");
    fs::read_to_string(file_path).expect("Failed to parse input file")
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<u32>().expect("Could not parse string to u32"))
                .next_tuple()
                .expect("Items must be in pair")
        })
        .unzip()
}

fn solve_distance(list1: &mut [u32], list2: &mut [u32]) -> u32 {
    list1.sort();
    list2.sort();
    list1.iter().zip(list2).map(|(x, y)| x.abs_diff(*y)).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_input_file();
    let (mut list1, mut list2) = parse(&input);
    let total_distance = solve_distance(&mut list1, &mut list2);

    println!("Total distance: {}", total_distance);

    Ok(())
}
