use std::fs::File;
use std::io::{self, BufRead};

fn calculate_calibration_value(line: &str) -> u32 {
    // Find first and last digit
    let first_digit = line.chars().find(|c| c.is_digit(10)).map(|c| c.to_digit(10)).unwrap_or(Some(0));
    println!("First digit: {:?}", first_digit);
    let last_digit = line.chars().rev().find(|c| c.is_digit(10)).map(|c| c.to_digit(10)).unwrap_or(Some(0));
    println!("Last digit: {:?}", last_digit);

    // Combine first and last digit - ex.: 4 *10 + 5 = 45
    match (first_digit, last_digit) {
        (Some(first), Some(last)) => first * 10 + last,
        _ => 0,
    }
}

fn main() {
    // Open input file
    // let file = File::open("example.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");

    // Read lines from the file
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    // Calculate the sum of the values
    let sum: u32 = lines.iter().map(|line| calculate_calibration_value(line)).sum();

    println!("Sum of calibration values: {}", sum);
}