use itertools::Itertools;
use std::{env, error::Error, fs, str::FromStr};

struct Report(Vec<u32>);

impl Report {
    fn is_safe(&self) -> bool {
        (self.is_strictly_increasing() || self.is_strictly_decreasing()) && self.is_in_range()
    }

    fn is_strictly_increasing(&self) -> bool {
        self.0.iter().tuple_windows().all(|(a, b)| a < b)
    }

    fn is_strictly_decreasing(&self) -> bool {
        self.0.iter().tuple_windows().all(|(a, b)| a > b)
    }

    fn is_in_range(&self) -> bool {
        self.0
            .iter()
            .tuple_windows()
            .all(|(a, b)| a.abs_diff(*b) <= 3)
    }
}

impl FromStr for Report {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<u32>, _>>()
            .map(Self)
    }
}

fn load_input_file() -> String {
    let file_path = env::args().nth(1).expect("No input file passed");
    fs::read_to_string(file_path).expect("Failed to parse input file")
}

fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|l| Report::from_str(l).expect("Could not parse line to Report"))
        .collect_vec()
}

fn solve_safe_reports(reports: &[Report]) -> u32 {
    reports.iter().filter(|r| r.is_safe()).count() as u32
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_input_file();
    let reports = parse(&input);
    let safe_reports_count = solve_safe_reports(&reports);

    println!("Safe reports: {}", safe_reports_count);

    Ok(())
}
